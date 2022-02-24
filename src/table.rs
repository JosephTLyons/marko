use std::cmp;
use std::collections::HashMap;

pub struct Table<'a> {
    headers: &'a [String],
    rows: &'a [HashMap<String, String>],
}

impl<'a> Table<'a> {
    pub fn from(headers: &'a [String], rows: &'a [HashMap<String, String>]) -> Table<'a> {
        Table { headers, rows }
    }

    pub fn get_column_width(&self, header: &str) -> usize {
        let width_of_largest_non_header_cell_in_column = self
            .rows
            .iter()
            .map(|row| row[header].as_str())
            .max_by_key(|value| value.len())
            .unwrap_or(header)
            .len();

        cmp::max(header.len(), width_of_largest_non_header_cell_in_column)
    }

    pub fn get_column_widths(&self) -> HashMap<String, usize> {
        let column_widths: HashMap<String, usize> = self
            .headers
            .iter()
            .map(|header| (header.clone(), self.get_column_width(header)))
            .collect();

        column_widths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn column_width_test() {
        let rows = vec![
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

        let table = Table::from(&headers, &rows);

        let column_width = table.get_column_width(&"Name".to_string());
        assert_eq!(column_width, 6);

        let column_width = table.get_column_width(&"Profession".to_string());
        assert_eq!(column_width, 10);
    }

    #[test]
    fn column_widths_test() {
        let rows = vec![
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

        let table = Table::from(&headers, &rows);
        let column_widths = table.get_column_widths();

        assert_eq!(column_widths["Name"], 6);
        assert_eq!(column_widths["Profession"], 10);
    }
}
