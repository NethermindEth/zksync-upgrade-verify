use crate::slots_names::bytes_to_hex_string;
use crate::{
    constants::{ENCODED_SELECTOR, FACETS_DATA_SLOT, FACETS_LEN_SLOT, ZERO, ZKSYNC_ERA},
    slots_names::add_one_to_big_number,
    strings::{
        get_dimond_storage_facets_string, get_facet_position_string,
        get_facet_to_selector_slots_string, get_selector_to_facet_slots0_string,
        get_selector_to_facet_slots1_string, DIAMOND_STORAGE_FACETS_LENGTH,
        DIAMOND_STORAGE_ISFROZEN, PRSISTENT_SLOT_NAMES,
    },
};
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{
        BlockId, BlockOverrides, Bytes, GethDebugBuiltInTracerConfig, GethDebugBuiltInTracerType,
        GethDebugTracerConfig, GethDebugTracerType, GethDebugTracingCallOptions, GethTrace,
        GethTraceFrame, PreStateConfig, PreStateFrame, H160, H256,
    },
    utils::keccak256,
};
use std::{
    collections::{BTreeMap, HashSet},
    str::FromStr,
};

// Split slot value into 4 bytes selectors and save in set
pub fn insert_selectors(selectors_set: &mut HashSet<[u8; 4]>, value: &[u8; 32]) {
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
pub fn print_slot_value_with_action<F>(
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
pub async fn print_and_get_max_slot_value(
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

// Trace call upgrade proposal transaction and return Diff
pub async fn get_pre_tracer_diff(
    provider: &Provider<Http>,
    tx_hash: &str,
) -> Result<(GethTrace, BlockId), String> {
    // Get upgrade proposal transaction
    let tx_hash: H256 = H256::from_str(tx_hash).map_err(|err| err.to_string())?;
    let in_tx = provider
        .get_transaction_receipt(tx_hash)
        .await
        .map_err(|err| err.to_string())?
        .ok_or("Transaction not found")?;

    // Check if it is upgrade proposal transaction
    // Governance address should emit TransparentOperationScheduled (index_topic_1 bytes32 _id, uint256 delay, tuple _operation)
    if in_tx.logs.len() == 0
        || in_tx.logs[0].address
            != H160::from_str("0x0b622a2061eaccae1c664ebc3e868b8438e03f61").unwrap()
        || in_tx.logs[0].topics[0]
            != H256::from_str("0x23bc9f5dc037eb49c162fd08c2a4d43dfe70063149e140d502273168da0a0625")
                .unwrap()
    {
        return Err("Wrong event topic".to_string());
    }

    // to governance
    let to = H160::from_str("0x0b622A2061EaccAE1c664eBC3E868b8438e03F61").unwrap();
    // from multisig
    let from = H160::from_str("0x4e4943346848c4867F81dFb37c4cA9C5715A7828").unwrap();

    // Get calldata from event log
    let mut call_data = in_tx.logs[0].data[32..].to_vec();
    call_data[31] = 0x20;
    let call = format!("0x74da756b{}", ethers::utils::hex::encode(call_data));
    let data: Bytes = Bytes::from_str(&call).unwrap();
    let call_tx = ethers::types::TransactionRequest::default()
        .to(to)
        .from(from)
        .data(data);

    // Override block timestamp
    let mut block_overides = BlockOverrides::default();
    // Set maximum i32 timestamp to pass any time check
    block_overides.time = Some((i32::MAX).into());
    // Select PreStateTracer
    let mut opts = GethDebugTracingCallOptions::default();
    opts.block_overrides = Some(block_overides);
    opts.tracing_options.tracer_config = Some(GethDebugTracerConfig::BuiltInTracer(
        GethDebugBuiltInTracerConfig::PreStateTracer(PreStateConfig {
            diff_mode: Some(true),
        }),
    ));
    opts.tracing_options.tracer = Some(GethDebugTracerType::BuiltInTracer(
        GethDebugBuiltInTracerType::PreStateTracer,
    ));
    // Get block number
    let block_id = BlockId::Number(
        (in_tx
            .block_number
            .ok_or_else(|| "No block number".to_string()))?
        .into(),
    );

    // Get debug trace
    let diff = provider
        .debug_trace_call(call_tx, Some(block_id), opts)
        .await
        .map_err(|err| err.to_string())?;
    Ok((diff, block_id))
}

// Decode storage
pub async fn parse_proposal_trace(tx_hash: &str, rpc_url: &str) -> Result<(), String> {
    //let tx_hash: H256 = H256::from_str(tx_hash).map_err(|err| err.to_string())?;
    // Create provider
    let provider = Provider::<Http>::try_from(rpc_url).map_err(|err| err.to_string())?;
    // Get debug call trace
    let (diff, block_id) = get_pre_tracer_diff(&provider, tx_hash).await?;
    // Print information from debug trace
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
        let mut facets_vec = Vec::new();
        // HashSet of all selectors from changed facets
        let mut selectors_set = HashSet::new();
        // Get data for DiamondStorage.facets[]
        let mut faucet_arr_slot = FACETS_DATA_SLOT;
        let mut insert_action = |pre: &H256, post: &H256| {
            if pre != &H256(ZERO) {
                facets_vec.push(pre.clone());
            }
            if post != &H256(ZERO) {
                facets_vec.push(post.clone());
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
        for facet in facets_vec {
            // Get DiamondStorage slot
            let mut encoded = ENCODED_SELECTOR.clone();
            // Get FacetToSelectors slot by offset
            encoded[63] += 1;
            // Encode data as ..address(key) . FacetToSelectorsStorageSlot
            encoded[0..32].copy_from_slice(facet.as_ref());
            // Get SelectorToFacet[facet_address].selectors length slot
            let array_hash = keccak256(encoded);
            // Get selector count for facet
            let selectors_count = print_and_get_max_slot_value(
                &H256(array_hash),
                pre,
                post,
                DIAMOND_STORAGE_FACETS_LENGTH,
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
        for (slot,value) in pre {
            if !known_slots.contains(&slot) {
                let title = format!("\x1b[38;5;117m0x{:02x}\x1b[0m",slot);
                let post_value = post.get(slot).unwrap_or(&H256(ZERO));
                println!(
                    "{}:\n  from:0x{:02x}\n    to:0x{:02x}",
                    title, value, post_value
                );
                known_slots.insert(*slot);
            }
        }
        for (slot,value) in post {
            if !known_slots.contains(&slot) {
                let title = format!("\x1b[38;5;117m0x{:02x}\x1b[0m",slot);
                println!(
                    "{}:\n  from:0x{:02x}\n    to:0x{:02x}",
                    title, H256(ZERO), value
                );
            }
        }
    } else {
        return Err("Can't get Diff from GethTrace".to_string());
    }
    Ok(())
}
