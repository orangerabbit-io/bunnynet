use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApiKey {
    pub id: Option<i64>,
    pub key: Option<String>,
    pub roles: Option<Vec<String>>,
}

#[derive(Debug, Tabled)]
pub struct ApiKeyRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "KEY")]
    pub key: String,
    #[tabled(rename = "ROLES")]
    pub roles: String,
}

impl From<&ApiKey> for ApiKeyRow {
    fn from(k: &ApiKey) -> Self {
        ApiKeyRow {
            id: k
                .id
                .map(|id| id.to_string())
                .unwrap_or_else(|| "-".to_string()),
            key: k.key.clone().unwrap_or_else(|| "-".to_string()),
            roles: k
                .roles
                .as_ref()
                .map(|r| r.join(", "))
                .unwrap_or_else(|| "-".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_api_key() {
        let json = r#"{
            "Id": 42,
            "Key": "abc-123-def",
            "Roles": ["admin", "billing"]
        }"#;

        let key: ApiKey = serde_json::from_str(json).unwrap();
        assert_eq!(key.id, Some(42));
        assert_eq!(key.key, Some("abc-123-def".to_string()));
        assert_eq!(
            key.roles,
            Some(vec!["admin".to_string(), "billing".to_string()])
        );
    }

    #[test]
    fn test_api_key_row_from_api_key() {
        let key = ApiKey {
            id: Some(42),
            key: Some("abc-123".to_string()),
            roles: Some(vec!["admin".to_string(), "billing".to_string()]),
        };

        let row = ApiKeyRow::from(&key);
        assert_eq!(row.id, "42");
        assert_eq!(row.key, "abc-123");
        assert_eq!(row.roles, "admin, billing");
    }
}
