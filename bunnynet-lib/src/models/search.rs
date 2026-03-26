use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchResults {
    pub query: Option<String>,
    pub total: Option<i64>,
    pub from: Option<i64>,
    pub size: Option<i64>,
    pub search_results: Option<Vec<SearchResultItem>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchResultItem {
    pub r#type: Option<String>,
    pub id: Option<i64>,
    pub name: Option<String>,
}

#[derive(Debug, Tabled)]
pub struct SearchResultItemRow {
    #[tabled(rename = "TYPE")]
    pub item_type: String,
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "NAME")]
    pub name: String,
}

impl From<&SearchResultItem> for SearchResultItemRow {
    fn from(item: &SearchResultItem) -> Self {
        SearchResultItemRow {
            item_type: item.r#type.clone().unwrap_or_else(|| "-".to_string()),
            id: item
                .id
                .map(|id| id.to_string())
                .unwrap_or_else(|| "-".to_string()),
            name: item.name.clone().unwrap_or_else(|| "-".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_search_results() {
        let json = r#"{
            "Query": "example",
            "Total": 2,
            "From": 0,
            "Size": 10,
            "SearchResults": [
                {
                    "Type": "cdn",
                    "Id": 123,
                    "Name": "my-zone"
                },
                {
                    "Type": "dns",
                    "Id": 456,
                    "Name": "example.com"
                }
            ]
        }"#;

        let results: SearchResults = serde_json::from_str(json).unwrap();
        assert_eq!(results.query, Some("example".to_string()));
        assert_eq!(results.total, Some(2));
        assert_eq!(results.from, Some(0));
        assert_eq!(results.size, Some(10));

        let items = results.search_results.unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].r#type, Some("cdn".to_string()));
        assert_eq!(items[0].id, Some(123));
        assert_eq!(items[0].name, Some("my-zone".to_string()));
        assert_eq!(items[1].r#type, Some("dns".to_string()));
    }

    #[test]
    fn test_search_result_item_row() {
        let item = SearchResultItem {
            r#type: Some("cdn".to_string()),
            id: Some(123),
            name: Some("my-zone".to_string()),
        };

        let row = SearchResultItemRow::from(&item);
        assert_eq!(row.item_type, "cdn");
        assert_eq!(row.id, "123");
        assert_eq!(row.name, "my-zone");
    }
}
