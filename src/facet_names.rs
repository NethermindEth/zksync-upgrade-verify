// get facet name from function selector
pub fn get_facet_name(selector: &[u8; 4]) -> Option<String> {
    match selector {
        // Admin Facet
        [14, 24, 182, 129] => Some("Admin".to_string()),
        // Governance Facet [Deprecated]
        [229, 139, 182, 57] => Some("Governance".to_string()),
        // Executor Facet
        [112, 31, 88, 197] => Some("Executor".to_string()),
        [12, 77, 216, 16] => Some("Executor".to_string()),
        // Mailbox Facet
        [108, 9, 96, 249] => Some("Mailbox".to_string()),
        // Getters Facet
        [205, 255, 172, 198] => Some("Getters".to_string()),
        // DiamondCut Facet [Deprecated]
        [115, 251, 146, 151] => Some("DiamondCut".to_string()),
        _ => None,
    }
}