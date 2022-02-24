mod table;

use std::collections::HashMap;
use table::Table;

// Convert everything to &str?
// fn create_markdown_table_lines<T>(rows: &[HashMap<String, T>]) -> Option<Vec<String>>
// where
//     T: Clone,
//     Vec<String>: FromIterator<T>,
// {
//     let mut headers: Vec<String> = rows.first()?.keys().cloned().collect();
//     headers.sort();

//     let separators: Vec<String> = headers.iter().map(|_| "-".to_string()).collect();

//     let create_row_string = |row: &Vec<String>| -> String { format!("| {} |", row.join(" | ")) };

//     let mut markdown_table_lines =
//         vec![create_row_string(&headers), create_row_string(&separators)];

//     for row in rows {
//         let row_values: Vec<String> = headers.iter().map(|header| row[header].clone()).collect();
//         markdown_table_lines.push(create_row_string(&row_values));
//     }

//     Some(markdown_table_lines)
// }

pub fn bold(text: &str) -> String {
    decorate_text_with(text, "**")
}

pub fn italicize(text: &str) -> String {
    decorate_text_with(text, "*")
}

fn decorate_text_with(text: &str, decoration: &str) -> String {
    format!("{}{}{}", decoration, text, decoration)
}

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

    let mut markdown_table_lines = vec![
        create_row_string(&padded_headers),
        create_row_string(&separators),
    ];

    for row in rows {
        let row_values: Vec<String> = headers
            .iter()
            .map(|header| create_padded_value(&row[header], value_pad_map[header]))
            .collect();
        markdown_table_lines.push(create_row_string(&row_values));
    }

    markdown_table_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bold_text() {
        assert_eq!("**Dog**".to_string(), bold("Dog"))
    }

    #[test]
    fn italicize_text() {
        assert_eq!("*Dog*".to_string(), italicize("Dog"))
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
