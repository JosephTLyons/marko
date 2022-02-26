mod table;

use std::collections::HashMap;
use table::Table;

pub trait Markdown {
    fn bold(&self) -> String;
    fn bullet(&self) -> String;
    fn code(&self) -> String;
    fn header(&self, level: u8) -> String;
    fn indent(&self, level: u8) -> String;
    fn italic(&self) -> String;
    fn link(&self, link: &str) -> String;
    fn quote(&self) -> String;
    fn strike(&self) -> String;
    fn task(&self, is_complete: bool) -> String;
}

impl<T: AsRef<str>> Markdown for T {
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

    fn header(&self, level: u8) -> String {
        match level {
            0 => self.as_ref().to_string(),
            _ => {
                let header_string = "#".repeat(level.into());
                let text = self.as_ref();
                format!("{header_string} {text}")
            }
        }
    }

    fn indent(&self, level: u8) -> String {
        match level {
            0 => self.as_ref().to_string(),
            _ => {
                let indent_string = "    ".repeat(level.into());
                let text = self.as_ref();
                format!("{indent_string}{text}")
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

    fn quote(&self) -> String {
        let text = self.as_ref();
        format!("> {text}")
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
    fn header_text_level_zero() {
        let text = "Dog";
        assert_eq!(text.header(0), text)
    }

    #[test]
    fn header_text_level_three() {
        let text = "Dog";
        assert_eq!(text.header(3), format!("### {text}"))
    }

    #[test]
    fn indent_text_level_zero() {
        let text = "Dog";
        assert_eq!(text.indent(0), text)
    }

    #[test]
    fn indent_text_level_three() {
        let text = "Dog";
        assert_eq!(text.indent(3), format!("            {text}"))
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
    fn quote_text() {
        let text = "Not all those who wander are lost";
        assert_eq!(text.quote(), format!("> {text}"))
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
    fn multiple_decorations_1() {
        let text = "Begin building markdown crate";
        let link = "https://github.com/JosephTLyons/marko";
        assert_eq!(
            text.bold().italic().link(link).task(true),
            format!("- [X] [***{text}***]({link})",)
        )
    }

    #[test]
    fn multiple_decorations_2() {
        let text = "Begin building markdown crate";
        assert_eq!(
            text.italic().header(3),
            format!("### *{text}*")
        )
    }

    #[test]
    fn divider_test() {
        assert_eq!("---", divider())
    }

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

// TODO
// Better logic for markdown table?
// Test for rows not matching headers
// Replace all HashMap []s with .get()
// Test names - what is the convention?
// Reduce clones
// Convert functions to trait of string, so . can be used?
// Possibly split table code and tests into its own file
// Numbered bullet point decoration
// Refactor non-pub decoration functions
// Test and make sure all code renders into markdown correctly
// Update changelog

// https://wordpress.com/support/markdown-quick-reference/
