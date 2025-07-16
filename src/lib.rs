//! # nerdicons-rs
//! Easily use Nerd Font icons in Rust.
//!
//! ## Example
//! ```rust
//! use nerdicons_rs::linux::RSAPPLE;
//! fn main() {
//!     println!("{}", RSAPPLE);
//! }
//! ```
pub mod icons;
pub use icons::*;
