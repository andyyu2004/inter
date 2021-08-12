#![allow(unused)]

mod ast;
mod lex;
mod parse;

pub use anyhow::Result;

#[macro_use]
extern crate anyhow;

fn main() {
    println!("Hello, world!");
}
