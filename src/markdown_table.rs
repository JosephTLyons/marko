mod table;
use table::Table;

use std::collections::HashMap;

pub fn create_markdown_table(headers: &[&str], rows: &[HashMap<&str, &str>]) -> Vec<String> {
    if headers.is_empty() || rows.is_empty() {
        return Vec::new();
    }

    let table = Table::from(headers, rows);

    let value_pad_map = table.get_column_widths();

    let padded_headers: Vec<_> = headers
        .iter()
        .map(|header| create_padded_value(header, value_pad_map[header]))
        .collect();

    let separators: Vec<_> = headers
        .iter()
        .map(|header| "-".repeat(value_pad_map[header]))
        .collect();

    let mut markdown_table = vec![
        create_row_string(&padded_headers),
        create_row_string(&separators),
    ];

    for row in rows {
        let row_values: Vec<_> = headers
            .iter()
            .map(|header| create_padded_value(row[header], value_pad_map[header]))
            .collect();

        markdown_table.push(create_row_string(&row_values));
    }

    markdown_table
}

fn create_padded_value(value: &str, pad_value: usize) -> String {
    format!("{:01$}", value, pad_value)
}

fn create_row_string(row: &[String]) -> String {
    format!("| {} |", row.join(" | "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_empty_headers() {
        let rows = [
            HashMap::from([("Name", "Joseph"), ("Profession", "Developer")]),
            HashMap::from([("Name", "Sam"), ("Profession", "Carpenter")]),
        ];

        let headers = [];

        let table_lines = create_markdown_table(&headers, &rows);
        let expected_output: Vec<String> = Vec::new();

        assert_eq!(table_lines, expected_output)
    }

    #[test]
    fn table_empty_rows() {
        let rows = [];
        let headers = ["Name", "Profession"];

        let table_lines = create_markdown_table(&headers, &rows);
        let expected_output: Vec<String> = Vec::new();

        assert_eq!(table_lines, expected_output)
    }

    #[test]
    fn table_with_values() {
        let rows = [
            HashMap::from([("Name", "Joseph"), ("Profession", "Developer")]),
            HashMap::from([("Name", "Sam"), ("Profession", "Carpenter")]),
        ];

        let mut headers: Vec<_> = rows.first().unwrap().keys().cloned().collect();
        headers.sort();

        let table_lines = create_markdown_table(&headers, &rows);

        let expected_output = [
            "| Name   | Profession |".to_string(),
            "| ------ | ---------- |".to_string(),
            "| Joseph | Developer  |".to_string(),
            "| Sam    | Carpenter  |".to_string(),
        ];

        assert_eq!(table_lines, expected_output);
    }
}
