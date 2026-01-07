/// URL ↔ Tree Path Synchronization
/// 
/// **Pattern:** Query param as source of truth for tree navigation
/// **Example:** /components?path=/workflows/workflow-1/step-1-2

/// Parse query param "path" into tree node IDs
pub fn parse_path_to_ids(path: &str) -> Vec<String> {
    path.trim_start_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// Build path string from tree node IDs
pub fn build_path_from_ids(ids: &[String]) -> String {
    if ids.is_empty() {
        String::new()
    } else {
        format!("/{}", ids.join("/"))
    }
}

/// Get last ID from path (selected node)
pub fn get_selected_id_from_path(path: &str) -> Option<String> {
    parse_path_to_ids(path).last().cloned()
}

/// Get parent IDs from path (nodes to expand)
pub fn get_parent_ids_from_path(path: &str) -> Vec<String> {
    let ids = parse_path_to_ids(path);
    if ids.len() <= 1 {
        vec![]
    } else {
        ids[..ids.len() - 1].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_path() {
        assert_eq!(
            parse_path_to_ids("/workflows/workflow-1/step-1-2"),
            vec!["workflows", "workflow-1", "step-1-2"]
        );
    }

    #[test]
    fn test_build_path() {
        assert_eq!(
            build_path_from_ids(&["workflows".into(), "workflow-1".into()]),
            "/workflows/workflow-1"
        );
    }
}
