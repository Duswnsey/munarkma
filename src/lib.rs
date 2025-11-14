//! A Namumark parser<br />
//! To start, make [Compiler] with [Compiler::from]<br />
//! Use [Compiler::parse] to parse.<br />
//! # Custom Macro
//! You can add Custon Macro using [Compiler::add_custom_macros]
//! # Example
//! ```rust
//!use munarkma::structs::Compiler;
//!fn main() {
//!    let compiler = Compiler::from(\\\\"Hello, Namumark!\\\\".to_string());
//!    println!(\\\\"{:#?}\\\\", compiler.array);
//!}
//!```

// use crate::structs::Compiler;

mod parse_third;
mod parser_first;
pub mod renderobjs;
pub mod structs;
#[cfg(test)]
mod tests;
// fn main () {
//     let mut compiler = Compiler::from("||<nopad><-19><nopad>adsf||".to_string());
//     compiler.parse();
//     println!("{:#?}", compiler.array);
// }
