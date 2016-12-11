mod table;

fn main() {
    let mut l = table::Table::new("test.txt".to_string());
    let n = l.write("A☑".to_string());
    println!("Array written is {:?}", n);
}

