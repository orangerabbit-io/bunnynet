use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaginatedList<T> {
    pub items: Vec<T>,
    pub current_page: i32,
    pub total_items: i32,
    pub has_more_items: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_paginated_list() {
        let json = r#"{"Items":[1,2,3],"CurrentPage":0,"TotalItems":10,"HasMoreItems":true}"#;
        let list: PaginatedList<i32> = serde_json::from_str(json).unwrap();
        assert_eq!(list.items, vec![1, 2, 3]);
        assert_eq!(list.current_page, 0);
        assert_eq!(list.total_items, 10);
        assert!(list.has_more_items);
    }
}
