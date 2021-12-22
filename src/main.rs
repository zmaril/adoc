#![feature(box_patterns)]
#![feature(str_split_once)]
extern crate colored; // not needed in Rust 2018
extern crate rustc_hash; // not needed in Rust 2018
extern crate rayon;
//pub mod p3radix;
pub mod p21;
//pub mod p22;

fn main() {
    p21::main();
    //p22::main();
}