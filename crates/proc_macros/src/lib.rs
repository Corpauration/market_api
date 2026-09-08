pub(crate) mod enum_from_csv;

/// Generates a Rust enum from a CSV file.
/// This accesses the CSV file at compile time, resolving the path relative to the Cargo manifest directory.
/// Usage:
/// ```rust
/// enum_from_csv! { pub enum MyEnum from "my/enum.csv" with header true }
/// ```
/// Properties accepted after `with`:
/// *   header: true | false - whether the CSV file has a header row
/// *   delimiter: char - the delimiter used in the CSV file (default: `,`)
#[proc_macro]
pub fn enum_from_csv(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    enum_from_csv::transform(input)
}
