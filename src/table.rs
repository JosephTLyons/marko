use std::cmp;
use std::collections::HashMap;

pub struct Table<'a> {
    headers: &'a [&'a str],
    rows: &'a [HashMap<&'a str, &'a str>],
}

impl<'a> Table<'a> {
    pub fn from(headers: &'a [&str], rows: &'a [HashMap<&str, &str>]) -> Table<'a> {
        Table { headers, rows }
    }

    pub fn get_column_width(&self, header: &str) -> usize {
        let width_of_largest_non_header_cell_in_column = self
            .rows
            .iter()
            .map(|row| row[header])
            .max_by_key(|value| value.len())
            .unwrap_or(header)
            .len();

        cmp::max(header.len(), width_of_largest_non_header_cell_in_column)
    }

    pub fn get_column_widths(&self) -> HashMap<&str, usize> {
        let column_widths: HashMap<&str, usize> = self
            .headers
            .iter()
            .map(|header| (*header, self.get_column_width(header)))
            .collect();

        column_widths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn column_width_test() {
        let rows = [
            HashMap::from([("Name", "Joseph"), ("Profession", "Developer")]),
            HashMap::from([("Name", "Sam"), ("Profession", "Carpenter")]),
        ];

        let mut headers: Vec<_> = rows.first().unwrap().keys().cloned().collect();
        headers.sort();

        let table = Table::from(&headers, &rows);

        let column_width = table.get_column_width(&"Name".to_string());
        assert_eq!(column_width, 6);

        let column_width = table.get_column_width(&"Profession".to_string());
        assert_eq!(column_width, 10);
    }

    #[test]
    fn column_widths_test() {
        let rows = [
            HashMap::from([("Name", "Joseph"), ("Profession", "Developer")]),
            HashMap::from([("Name", "Sam"), ("Profession", "Carpenter")]),
        ];

        let mut headers: Vec<_> = rows.first().unwrap().keys().cloned().collect();
        headers.sort();

        let table = Table::from(&headers, &rows);
        let column_widths = table.get_column_widths();

        assert_eq!(column_widths["Name"], 6);
        assert_eq!(column_widths["Profession"], 10);
    }
}
