//@ aux-build:issue-100204-aux.rs
//@ build-aux-docs
//@ ignore-cross-compile

#![crate_name="second"]

extern crate first;

pub mod prelude {}

// @has first/struct.Bot.html '//h4[@class="code-header"]' 'pub fn new() -> Bot'
// @has second/struct.Bot.html '//h4[@class="code-header"]' 'pub fn new() -> Bot'
#[doc(inline)]
pub use first::*;
