use tabled::settings::Style;
use tabled::{Table, Tabled};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputMode {
    Table,
    Json,
}

impl OutputMode {
    pub fn from_json_flag(json: bool) -> Self {
        if json {
            OutputMode::Json
        } else {
            OutputMode::Table
        }
    }
}

pub fn print_json(value: &serde_json::Value) {
    println!(
        "{}",
        serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
    );
}

pub fn print_table<T: Tabled>(items: &[T]) {
    if items.is_empty() {
        println!("No results.");
        return;
    }
    let mut table = Table::new(items);
    table.with(Style::markdown());
    println!("{}", table);
}

pub fn print_kv(pairs: &[(&str, String)]) {
    let max_key_len = pairs.iter().map(|(k, _)| k.len()).max().unwrap_or(0);
    for (key, value) in pairs {
        println!("{:>width$}:  {}", key, value, width = max_key_len);
    }
}

pub fn print_confirm(message: &str) {
    println!("{}", message);
}

pub fn print_pagination(current_page: i32, total_items: i32, has_more_items: bool) {
    let suffix = if has_more_items {
        ", more available"
    } else {
        ""
    };
    println!(
        "Page {} ({} total items{})",
        current_page, total_items, suffix
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_mode_from_flag() {
        assert_eq!(OutputMode::from_json_flag(true), OutputMode::Json);
        assert_eq!(OutputMode::from_json_flag(false), OutputMode::Table);
    }
}
