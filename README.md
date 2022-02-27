# marko

Programmtically format text with [Markdown](https://en.wikipedia.org/wiki/Markdown) syntax, with [marko](https://crates.io/crates/marko)!

```rust
use marko::{self, Markdown};
use std::collections::HashMap;

fn main() {
    let markdown_text = "Finish marko"
        .bold()
        .link("https://github.com/JosephTLyons/marko")
        .task(false);

    println!("{}", markdown_text);

    // - [ ] [**Finish marko**](https://github.com/JosephTLyons/marko)

    println!("{}", marko::divider());

    // ---

    let rows = [
        HashMap::from([
            ("Name", "Joseph"),
            ("Profession", "Developer"),
            ("Age", "31"),
            ("State", "Indiana"),
        ]),
        HashMap::from([
            ("Name", "Sam"),
            ("Profession", "Carpenter"),
            ("Age", "31"),
            ("State", "Arizona"),
        ]),
        HashMap::from([
            ("Name", "Seth"),
            ("Profession", "Fabricator"),
            ("Age", "30"),
            ("State", "Ohio"),
        ]),
        HashMap::from([
            ("Name", "Danny"),
            ("Profession", "Guitarist"),
            ("Age", "31"),
            ("State", "Indiana"),
        ]),
    ];

    let mut headers: Vec<_> = rows.first().unwrap().keys().cloned().collect();
    headers.sort();

    for row in marko::create_markdown_table(&headers, &rows) {
        println!("{row}");
    }

    // | Age | Name   | Profession | State   |
    // | --- | ------ | ---------- | ------- |
    // | 31  | Joseph | Developer  | Indiana |
    // | 31  | Sam    | Carpenter  | Arizona |
    // | 30  | Seth   | Fabricator | Ohio    |
    // | 31  | Danny  | Guitarist  | Indiana |
}
```
