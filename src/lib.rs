mod table;

use std::collections::HashMap;
use table::Table;

// TODO - rename this trait
pub trait TextMarkup {
    fn bold(&self) -> String;
    fn bullet(&self) -> String;
    fn code(&self) -> String;
    fn italicize(&self) -> String;
    fn link(&self, link: &str) -> String;
    fn strike(&self) -> String;
    fn task(&self, is_checked: bool) -> String;
}

impl TextMarkup for &str {
    fn bold(&self) -> String {
        decorate_text_with(self, "**")
    }

    fn bullet(&self) -> String {
        format!("- {}", self)
    }

    fn code(&self) -> String {
        decorate_text_with(self, "`")
    }

    fn italicize(&self) -> String {
        decorate_text_with(self, "*")
    }

    fn link(&self, link: &str) -> String {
        format!("[{}]({})", self, link)
    }

    fn strike(&self) -> String {
        decorate_text_with(self, "~~")
    }

    fn task(&self, is_complete: bool) -> String {
        format!("- [{}] {}", if is_complete { "X" } else { " " }, self)
    }
}

fn decorate_text_with(text: &str, decoration: &str) -> String {
    format!("{}{}{}", decoration, text, decoration)
}

pub fn divider() -> &'static str {
    "---"
}

// pub fn image() {}

// pub fn create_unorderd_bullet_point_list() {}

// pub fn create_ordered_bullet_point_list() {}

pub fn create_formatted_markdown_table(
    headers: &[String],
    rows: &[HashMap<String, String>],
) -> Vec<String> {
    if headers.is_empty() || rows.is_empty() {
        return Vec::new();
    }

    let value_pad_map = Table::from(headers, rows).get_column_widths();

    let create_padded_value =
        |value: &String, pad_value: usize| -> String { format!("{:01$}", value, pad_value) };

    let padded_headers = headers
        .iter()
        .map(|header| create_padded_value(header, value_pad_map[header]))
        .collect();

    let separators: Vec<String> = headers
        .iter()
        .map(|header| "-".repeat(value_pad_map[header]))
        .collect();

    let create_row_string = |row: &Vec<String>| -> String { format!("| {} |", row.join(" | ")) };

    let mut markdown_table = vec![
        create_row_string(&padded_headers),
        create_row_string(&separators),
    ];

    for row in rows {
        let row_values: Vec<String> = headers
            .iter()
            .map(|header| create_padded_value(&row[header], value_pad_map[header]))
            .collect();
        markdown_table.push(create_row_string(&row_values));
    }

    markdown_table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bold_text() {
        assert_eq!("Dog".bold(), "**Dog**",)
    }

    #[test]
    fn bullet_text() {
        assert_eq!("Dog".bullet(), "- Dog",)
    }

    #[test]
    fn code_text() {
        assert_eq!("Dog".code(), "`Dog`",)
    }

    #[test]
    fn italicize_text() {
        assert_eq!("Dog".italicize(), "*Dog*",)
    }

    #[test]
    fn link_text() {
        assert_eq!("Dog".link("www.apple.com"), "[Dog](www.apple.com)",)
    }

    #[test]
    fn strike_text() {
        assert_eq!("Dog".strike(), "~~Dog~~",)
    }

    #[test]
    fn task_complete() {
        assert_eq!("Finish refactor".task(true), "- [X] Finish refactor",)
    }

    #[test]
    fn task_not_complete() {
        assert_eq!("Finish refactor".task(false), "- [ ] Finish refactor",)
    }

    #[test]
    fn divider_test() {
        assert_eq!("---", divider())
    }

    #[test]
    fn empty_headers() {
        let rows = [
            HashMap::from([
                ("Name".to_string(), "Joseph".to_string()),
                ("Profession".to_string(), "Developer".to_string()),
            ]),
            HashMap::from([
                ("Name".to_string(), "Sam".to_string()),
                ("Profession".to_string(), "Carpenter".to_string()),
            ]),
        ];

        let headers = [];

        let table_lines = create_formatted_markdown_table(&headers, &rows);
        let expected_output: Vec<String> = Vec::new();

        assert_eq!(table_lines, expected_output)
    }

    #[test]
    fn empty_rows() {
        let rows = [];
        let headers = ["Name".to_string(), "Profession".to_string()];

        let table_lines = create_formatted_markdown_table(&headers, &rows);
        let expected_output: Vec<String> = Vec::new();

        assert_eq!(table_lines, expected_output)
    }

    #[test]
    fn non_empty_table_test() {
        let rows = [
            HashMap::from([
                ("Name".to_string(), "Joseph".to_string()),
                ("Profession".to_string(), "Developer".to_string()),
            ]),
            HashMap::from([
                ("Name".to_string(), "Sam".to_string()),
                ("Profession".to_string(), "Carpenter".to_string()),
            ]),
        ];

        let mut headers: Vec<String> = rows.first().unwrap().keys().cloned().collect();
        headers.sort();

        let table_lines = create_formatted_markdown_table(&headers, &rows);

        let expected_output = [
            "| Name   | Profession |".to_string(),
            "| ------ | ---------- |".to_string(),
            "| Joseph | Developer  |".to_string(),
            "| Sam    | Carpenter  |".to_string(),
        ];

        assert_eq!(table_lines, expected_output);
    }
}

// TODO
// Better logic
// Only keep one function or both?
// Test for rows not matching headers
// Other markdown functions?
// Clean table function
// Replace all HashMap []s with .get()
// Test names - what is the convention?
// Reduce clones
// Convert functions to trait of string, so . can be used?
