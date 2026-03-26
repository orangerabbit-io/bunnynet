use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Country {
    pub name: Option<String>,
    pub iso_code: Option<String>,
    #[serde(rename = "IsEU")]
    pub is_eu: Option<bool>,
    pub tax_rate: Option<f64>,
    pub tax_prefix: Option<String>,
    pub flag_url: Option<String>,
    pub pop_list: Option<Vec<String>>,
}

#[derive(Debug, Tabled)]
pub struct CountryRow {
    #[tabled(rename = "ISO CODE")]
    pub iso_code: String,
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(rename = "EU")]
    pub eu: String,
    #[tabled(rename = "TAX RATE")]
    pub tax_rate: String,
}

impl From<&Country> for CountryRow {
    fn from(c: &Country) -> Self {
        CountryRow {
            iso_code: c.iso_code.clone().unwrap_or_else(|| "-".to_string()),
            name: c.name.clone().unwrap_or_else(|| "-".to_string()),
            eu: c
                .is_eu
                .map(|b| if b { "yes" } else { "no" }.to_string())
                .unwrap_or_else(|| "-".to_string()),
            tax_rate: c
                .tax_rate
                .map(|r| format!("{:.1}%", r))
                .unwrap_or_else(|| "-".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_country() {
        let json = r#"{
            "Name": "Germany",
            "IsoCode": "DE",
            "IsEU": true,
            "TaxRate": 19.0,
            "TaxPrefix": "DE",
            "FlagUrl": "https://bunnycdn.com/flags/de.png",
            "PopList": ["DE-FRA", "DE-HAM"]
        }"#;

        let country: Country = serde_json::from_str(json).unwrap();
        assert_eq!(country.name, Some("Germany".to_string()));
        assert_eq!(country.iso_code, Some("DE".to_string()));
        assert_eq!(country.is_eu, Some(true));
        assert_eq!(country.tax_rate, Some(19.0));
        assert_eq!(
            country.pop_list,
            Some(vec!["DE-FRA".to_string(), "DE-HAM".to_string()])
        );
    }

    #[test]
    fn test_country_row_from_country() {
        let country = Country {
            name: Some("Germany".to_string()),
            iso_code: Some("DE".to_string()),
            is_eu: Some(true),
            tax_rate: Some(19.0),
            tax_prefix: Some("DE".to_string()),
            flag_url: None,
            pop_list: None,
        };

        let row = CountryRow::from(&country);
        assert_eq!(row.iso_code, "DE");
        assert_eq!(row.eu, "yes");
        assert_eq!(row.tax_rate, "19.0%");
    }
}
