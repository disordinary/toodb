#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
mod table;
mod log;

fn main() {
    let mut ll = log::Log::new("test.log".to_string());
    ll.read();
}

