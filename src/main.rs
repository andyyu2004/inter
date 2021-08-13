#![allow(unused)]

mod ast;
mod bytecode;
mod compiler;
mod lex;
#[macro_use]
mod parse;
mod value;
mod vm;

pub use anyhow::Result;

#[macro_use]
extern crate anyhow;

fn main() {
    println!("Hello, world!");
}
