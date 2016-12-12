#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
mod table;
mod log;

fn main() {
//    let mut l = table::Table::new("test.txt".to_string());
//    let n = l.write("A☑".to_string());
//    println!("Array written is {:?}", n);
    let mut ll = log::Log::new("test.log".to_string());
    ll.read();
    //ll.read();
    //println!("{:?}", ll.add_new("TEST🍻".to_string(), 100, 100));
}

