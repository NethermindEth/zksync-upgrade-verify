use crate::utils::bytes_to_hex_string;
use crate::strings::get_facet_to_selectors_length;
use crate::{
    constants::{ENCODED_SELECTOR, FACETS_DATA_SLOT, FACETS_LEN_SLOT, ZERO, ZKSYNC_ERA},
    utils::add_one_to_big_number,
    strings::{
        get_dimond_storage_facets_string, get_facet_position_string,
        get_facet_to_selector_slots_string, get_selector_to_facet_slots0_string,
        get_selector_to_facet_slots1_string, DIAMOND_STORAGE_FACETS_LENGTH,
        DIAMOND_STORAGE_ISFROZEN, PRSISTENT_SLOT_NAMES,
    },
};
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{BlockId, GethTrace, GethTraceFrame, PreStateFrame, H256},
    utils::keccak256,
};
use std::collections::{BTreeMap, HashSet};

// Split slot value into 4 bytes selectors and save in set
fn insert_selectors(selectors_set: &mut HashSet<[u8; 4]>, value: &[u8; 32]) {
    for chunk in value.chunks_exact(4).rev() {
        if chunk.iter().any(|&x| x != 0) {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(chunk);
            selectors_set.insert(bytes);
        } else {
            break;
        }
    }
}

// Print slot value and call action if needed
fn print_slot_value_with_action<F>(
    slot: &H256,
    pre: &BTreeMap<H256, H256>,
    post: &BTreeMap<H256, H256>,
    title: &str,
    action: Option<F>,
) where
    F: FnMut(&H256, &H256),
{
    let pre_value = pre.get(&slot);
    let post_value = post.get(&slot);
    if pre_value.is_some() || post_value.is_some() {
        let zero = H256(ZERO);
        let pre_value = pre_value.unwrap_or(&zero);
        let post_value = post_value.unwrap_or(&zero);
        println!(
            "{}:\n  from:0x{:02x}\n    to:0x{:02x}",
            title, pre_value, post_value
        );
        if let Some(mut action) = action {
            action(&pre_value, &post_value);
        }
    }
}

// Print slot value and return max value
// If slot value is not changed, call proveider to get storage at that slot with specified block
async fn print_and_get_max_slot_value(
    slot: &H256,
    pre: &BTreeMap<H256, H256>,
    post: &BTreeMap<H256, H256>,
    title: &str,
    provider: &Provider<Http>,
    block_id: BlockId,
) -> Result<u64, String> {
    let pre_value = pre.get(slot);
    let post_value = post.get(slot);
    let max_value = if pre_value.is_some() || post_value.is_some() {
        let zero = H256(ZERO);
        let pre_value = pre_value.unwrap_or(&zero);
        let post_value = post_value.unwrap_or(&zero);
        println!(
            "{}:\n  from:0x{:02x}\n    to:0x{:02x}",
            title, pre_value, post_value
        );
        pre_value.to_low_u64_be().max(post_value.to_low_u64_be())
    } else {
        provider
            .get_storage_at(ZKSYNC_ERA, *slot, Some(block_id))
            .await
            .map_err(|err| err.to_string())?
            .to_low_u64_be()
    };
    return Ok(max_value);
}

// Parse diff and print information from debug trace
pub async fn parse_storage_diff(
    provider: &Provider<Http>,
    diff: &GethTrace,
    block_id: BlockId,
) -> Result<(), String> {
    if let GethTrace::Known(GethTraceFrame::PreStateTracer(PreStateFrame::Diff(diff_val))) = diff {
        // Get Diff for zkSync Era contract
        let pre = diff_val
            .pre
            .get(&ZKSYNC_ERA)
            .ok_or_else(|| "No pre data for zkSync Era contract".to_string())?;
        let post = diff_val
            .post
            .get(&ZKSYNC_ERA)
            .ok_or_else(|| "No post data for zkSync Era contract".to_string())?;

        // Get storage value
        let pre = pre
            .storage
            .as_ref()
            .ok_or_else(|| "No pre storage data for zkSync Era contract".to_string())?;
        let post = post
            .storage
            .as_ref()
            .ok_or_else(|| "No post storage data for zkSync Era contract".to_string())?;

        let mut known_slots = HashSet::new();
        println!("\x1b[38;5;49mDiamond Proxy storage changed:\x1b[0m");
        // Search changes in persistent slots
        for i in 0..PRSISTENT_SLOT_NAMES.len() {
            let slot = H256::from_low_u64_be(i as u64);
            print_slot_value_with_action(
                &slot,
                pre,
                post,
                PRSISTENT_SLOT_NAMES[i],
                None::<fn(&H256, &H256)>,
            );
            known_slots.insert(slot);
        }

        // Search changes in DiamondStorage slots
        // Find DiamondStorage.facets[] length
        let facets_count = print_and_get_max_slot_value(
            &H256(FACETS_LEN_SLOT),
            pre,
            post,
            DIAMOND_STORAGE_FACETS_LENGTH,
            &provider,
            block_id,
        )
        .await?;
        known_slots.insert(H256(FACETS_LEN_SLOT));
        // Vector to store all facets changed during the update
        let mut facets_set = HashSet::new();
        // HashSet of all selectors from changed facets
        let mut selectors_set = HashSet::new();
        // Get data for DiamondStorage.facets[]
        let mut faucet_arr_slot = FACETS_DATA_SLOT;
        let mut insert_action = |pre: &H256, post: &H256| {
            if pre != &H256(ZERO) {
                facets_set.insert(pre.clone());
            }
            if post != &H256(ZERO) {
                facets_set.insert(post.clone());
            }
        };

        for i in 0..facets_count {
            let title = get_dimond_storage_facets_string(i);
            print_slot_value_with_action(
                &H256(faucet_arr_slot),
                pre,
                post,
                &title,
                Some(&mut insert_action),
            );
            known_slots.insert(H256(faucet_arr_slot));
            // increment faucet_arr_slot
            add_one_to_big_number(&mut faucet_arr_slot);
        }

        // Check DiamondStorage.isFrozen slot.
        let mut slot = FACETS_LEN_SLOT.clone();
        slot[31] += 1;
        let title = DIAMOND_STORAGE_ISFROZEN;
        print_slot_value_with_action(&H256(slot), pre, post, &title, None::<fn(&H256, &H256)>);
        known_slots.insert(H256(slot));
        // For every faccet we need to find FacetToSelectors and save all selectors
        for facet in facets_set {
            // Get DiamondStorage slot
            let mut encoded = ENCODED_SELECTOR.clone();
            // Get FacetToSelectors slot by offset
            encoded[63] += 1;
            // Encode data as ..address(key) . FacetToSelectorsStorageSlot
            encoded[0..32].copy_from_slice(facet.as_ref());
            // Get SelectorToFacet[facet_address].selectors length slot
            let array_hash = keccak256(encoded);
            let title = get_facet_to_selectors_length(&bytes_to_hex_string(&facet.0[12..32]));
            // Get selector count for facet
            let selectors_count = print_and_get_max_slot_value(
                &H256(array_hash),
                pre,
                post,
                &title,
                &provider,
                block_id,
            )
            .await?;
            known_slots.insert(H256(array_hash));
            // Get SelectorToFacet[facet_address].facetPosition slot
            let mut facet_position_hash = array_hash.clone();
            add_one_to_big_number(&mut facet_position_hash);
            let title = get_facet_position_string(&bytes_to_hex_string(&facet.0[12..32]));
            print_slot_value_with_action(
                &H256(facet_position_hash),
                pre,
                post,
                &title,
                None::<fn(&H256, &H256)>,
            );
            known_slots.insert(H256(facet_position_hash));
            // Get SelectorToFacet[facet_address].selectors data slot
            let mut value_hash = keccak256(array_hash);
            // Find slot numbers to store all selectors. We store max 8 selectors per slot.
            let max_selectors: i32 = selectors_count as i32 / 8 + 1;
            let mut insert_action = |pre: &H256, post: &H256| {
                insert_selectors(&mut selectors_set, &pre.0);
                insert_selectors(&mut selectors_set, &post.0);
            };

            for j in 0..max_selectors {
                let title = get_facet_to_selector_slots_string(
                    &bytes_to_hex_string(&facet.0[12..32]),
                    j + 1,
                    max_selectors,
                );
                print_slot_value_with_action(
                    &H256(value_hash),
                    pre,
                    post,
                    &title,
                    Some(&mut insert_action),
                );
                known_slots.insert(H256(value_hash));
                // increment value_hash
                add_one_to_big_number(&mut value_hash);
            }
        }

        // Find all selectors to facet
        let mut encoded = ENCODED_SELECTOR.clone();
        for selector in selectors_set {
            // Find selector to facet
            // Encode data as byte4(key).. . SelectorToFacetStorageSlot
            encoded[..4].copy_from_slice(&selector);
            // Get SelectorToFacet[selector].facet slot
            let mut slot = keccak256(encoded);
            let title = get_selector_to_facet_slots0_string(&bytes_to_hex_string(&selector));
            print_slot_value_with_action(&H256(slot), pre, post, &title, None::<fn(&H256, &H256)>);
            known_slots.insert(H256(slot));
            // FacetToSelectors[selector]
            // Get slot for selectorPosition and isFreezable
            add_one_to_big_number(&mut slot);
            let title = get_selector_to_facet_slots1_string(&bytes_to_hex_string(&selector));
            print_slot_value_with_action(&H256(slot), pre, post, &title, None::<fn(&H256, &H256)>);
        }

        // Print unknown slots
        for (slot, value) in pre {
            if !known_slots.contains(&slot) {
                let title = format!("\x1b[38;5;117m0x{:02x}\x1b[0m", slot);
                let post_value = post.get(slot).unwrap_or(&H256(ZERO));
                println!(
                    "{}:\n  from:0x{:02x}\n    to:0x{:02x}",
                    title, value, post_value
                );
                known_slots.insert(*slot);
            }
        }
        for (slot, value) in post {
            if !known_slots.contains(&slot) {
                let title = format!("\x1b[38;5;117m0x{:02x}\x1b[0m", slot);
                println!(
                    "{}:\n  from:0x{:02x}\n    to:0x{:02x}",
                    title,
                    H256(ZERO),
                    value
                );
            }
        }
    } else {
        return Err("Can't get Diff from GethTrace".to_string());
    }
    Ok(())
}
