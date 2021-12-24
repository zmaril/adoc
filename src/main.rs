#![feature(box_patterns)]
#![feature(str_split_once)]
#![feature(destructuring_assignment)]
extern crate colored; // not needed in Rust 2018
extern crate rustc_hash; // not needed in Rust 2018
extern crate rayon;
extern crate cached;
#[macro_use] extern crate maplit;

pub mod p22;
pub mod p22cc;
pub mod p23;

fn main() {
    //p22::main();
   p22cc::main();
   //p23::main()
}