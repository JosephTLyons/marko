mod table;
use table::Table;

use std::collections::HashMap;

pub enum ColumnAlignment {
    Left,
    Center,
    Right,
}

pub fn create_markdown_table<'a>(
    headers: &[&str],
    rows: &'a [HashMap<&str, &str>],
    column_alignment_map_option: &Option<HashMap<&'a str, Option<ColumnAlignment>>>,
) -> Vec<String> {
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
        .map(|header| {
            let pad_value = value_pad_map[header];

            match &column_alignment_map_option {
                Some(column_alignment_map) => match &column_alignment_map[header] {
                    Some(ColumnAlignment::Left) => format!(":{}", "-".repeat(pad_value - 1)),
                    Some(ColumnAlignment::Center) => format!(":{}:", "-".repeat(pad_value - 2)),
                    Some(ColumnAlignment::Right) => format!("{}:", "-".repeat(pad_value - 1)),
                    None => "-".repeat(pad_value),
                },
                None => "-".repeat(pad_value),
            }
        })
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

        let table_lines = create_markdown_table(&headers, &rows, &None);
        let expected_output: Vec<String> = Vec::new();

        assert_eq!(table_lines, expected_output)
    }

    #[test]
    fn table_empty_rows() {
        let rows = [];
        let headers = ["Name", "Profession"];

        let table_lines = create_markdown_table(&headers, &rows, &None);
        let expected_output: Vec<String> = Vec::new();

        assert_eq!(table_lines, expected_output)
    }

    #[test]
    fn table_with_values() {
        let headers = ["Name", "Age", "Profession", "State"];

        let rows = [
            HashMap::from([
                ("Name", "Joseph"),
                ("Age", "31"),
                ("Profession", "Developer"),
                ("State", "Indiana"),
            ]),
            HashMap::from([
                ("Name", "Sam"),
                ("Age", "31"),
                ("Profession", "Carpenter"),
                ("State", "Arizona"),
            ]),
            HashMap::from([
                ("Name", "Seth"),
                ("Age", "31"),
                ("Profession", "Fabricator"),
                ("State", "Ohio"),
            ]),
            HashMap::from([
                ("Name", "Danny"),
                ("Age", "31"),
                ("Profession", "Guitarist"),
                ("State", "Indiana"),
            ]),
        ];

        let column_alignment_map_option = HashMap::from([
            ("Name", None),
            ("Age", Some(ColumnAlignment::Left)),
            ("Profession", Some(ColumnAlignment::Center)),
            ("State", Some(ColumnAlignment::Right)),
        ]);

        let table_lines =
            create_markdown_table(&headers, &rows, &Some(column_alignment_map_option));

        let expected_output = [
            "| Name   | Age | Profession | State   |",
            "| ------ | :-- | :--------: | ------: |",
            "| Joseph | 31  | Developer  | Indiana |",
            "| Sam    | 31  | Carpenter  | Arizona |",
            "| Seth   | 31  | Fabricator | Ohio    |",
            "| Danny  | 31  | Guitarist  | Indiana |",
        ];

        assert_eq!(table_lines, expected_output);
    }
}
