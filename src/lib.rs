mod markdown_table;
mod markdown;

pub use markdown_table::create_markdown_table;
pub use markdown::Markdown;
pub use markdown::divider;

// TODO
// Better logic for markdown table?
// Test for rows not matching headers
// Replace all HashMap []s with .get()
// Test names - what is the convention?
// Reduce clones
// Numbered bullet point decoration
// Refactor non-pub decoration functions
// Test and make sure all code renders into markdown correctly
// Update changelog

// https://wordpress.com/support/markdown-quick-reference/
