#![feature(box_patterns)]
#![feature(str_split_once)]
extern crate colored; // not needed in Rust 2018

//pub mod p3radix;
pub mod p16;

fn main() {
    p16::main();
}