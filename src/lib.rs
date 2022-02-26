mod table;

use std::collections::HashMap;
use table::Table;

// TODO - rename this trait
pub trait TextMarkup {
    fn bold(&self) -> String;
    fn bullet(&self) -> String;
    fn code(&self) -> String;
    fn header(&self, header_depth: u8) -> String;
    fn italic(&self) -> String;
    fn link(&self, link: &str) -> String;
    fn strike(&self) -> String;
    fn task(&self, is_checked: bool) -> String;
}

impl<T: AsRef<str>> TextMarkup for T {
    fn bold(&self) -> String {
        decorate_text_with(self.as_ref(), "**")
    }

    fn bullet(&self) -> String {
        let text = self.as_ref();
        format!("- {text}")
    }

    fn code(&self) -> String {
        decorate_text_with(self.as_ref(), "`")
    }

    fn header(&self, header_depth: u8) -> String {
        match header_depth {
            0 => self.as_ref().to_string(),
            _ => {
                let header_string = "#".repeat(header_depth.into());
                let text = self.as_ref();
                format!("{header_string} {text}")
            }
        }
    }

    fn italic(&self) -> String {
        decorate_text_with(self.as_ref(), "*")
    }

    fn link(&self, link: &str) -> String {
        let text = self.as_ref();
        format!("[{text}]({link})")
    }

    fn strike(&self) -> String {
        decorate_text_with(self.as_ref(), "~~")
    }

    fn task(&self, is_complete: bool) -> String {
        let is_complete_symbol = if is_complete { "X" } else { " " };
        let text = self.as_ref();
        format!("- [{is_complete_symbol}] {text}")
    }
}

fn decorate_text_with<T: AsRef<str>>(text: T, decoration: &str) -> String {
    let text = text.as_ref();
    format!("{decoration}{text}{decoration}")
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
        let text = "Dog";
        assert_eq!(text.bold(), format!("**{text}**"))
    }

    #[test]
    fn bullet_text() {
        let text = "Dog";
        assert_eq!(text.bullet(), format!("- {text}"))
    }

    #[test]
    fn code_text() {
        let text = "Dog";
        assert_eq!(text.code(), format!("`{text}`"))
    }

    #[test]
    fn header_text_depth_of_zero() {
        let text = "Dog";
        assert_eq!(text.header(0), text)
    }

    #[test]
    fn header_text_depth_of_three() {
        let text = "Dog";
        assert_eq!(text.header(3), format!("### {text}"))
    }

    #[test]
    fn italicize_text() {
        let text = "Dog";
        assert_eq!(text.italic(), format!("*{text}*"))
    }

    #[test]
    fn link_text() {
        let text = "Apple";
        let link = "www.apple.com";
        assert_eq!(text.link(link), format!("[{text}]({link})"))
    }

    #[test]
    fn strike_text() {
        let text = "Dog";
        assert_eq!(text.strike(), format!("~~{text}~~"))
    }

    #[test]
    fn task_complete() {
        let text = "Finish refactor";
        assert_eq!(text.task(true), format!("- [X] {text}"))
    }

    #[test]
    fn task_not_complete() {
        let text = "Finish refactor";
        assert_eq!(text.task(false), format!("- [ ] {text}"))
    }

    #[test]
    fn many_decorations() {
        let text = "Begin building markdown crate";
        let link = "https://github.com/JosephTLyons/markdown";
        assert_eq!(
            text.bold().italic().link(link).task(true),
            format!("- [X] [***{text}***]({link})",)
        )
    }

    #[test]
    fn divider_test() {
        assert_eq!("---", divider())
    }

    #[test]
    fn table_empty_headers() {
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
    fn table_empty_rows() {
        let rows = [];
        let headers = ["Name".to_string(), "Profession".to_string()];

        let table_lines = create_formatted_markdown_table(&headers, &rows);
        let expected_output: Vec<String> = Vec::new();

        assert_eq!(table_lines, expected_output)
    }

    #[test]
    fn table_with_values() {
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
// Better logic for markdown table?
// Test for rows not matching headers
// Replace all HashMap []s with .get()
// Test names - what is the convention?
// Reduce clones
// Convert functions to trait of string, so . can be used?
// Possibly split table code and tests into its own file
