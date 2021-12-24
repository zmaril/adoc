#![feature(box_patterns)]
#![feature(str_split_once)]
#![feature(destructuring_assignment)]
extern crate colored; // not needed in Rust 2018
extern crate rustc_hash; // not needed in Rust 2018
extern crate rayon;
extern crate cached;
#[macro_use] extern crate maplit;
extern crate itertools;
extern crate num_format;

//pub mod p22;
//pub mod p22cc;
//pub mod p23;
pub mod p24;

fn main() {
    //p22::main();
   //p22cc::main();
   //p23::main()
   p24::main();
}