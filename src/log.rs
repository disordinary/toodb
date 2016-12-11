use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Seek;
use std::io::SeekFrom;


#[derive(Debug)]
pub struct Record {
    offset: usize,
    size: usize,
}

pub struct Table {
    file: fs::File,
    size: usize,
}

impl Table {
    pub fn new(file_name: String) -> Table {
        let mut option = OpenOptions::new();
        option.read(true);
        option.write(false);
        option.append(true);
        option.create(true);
        let mut table = Table {
            file: option.open(file_name).unwrap(),
            size: 0,
        };
        table.size = table.file.seek(SeekFrom::End(0)).unwrap() as usize;
        table
    }

    pub fn write(&mut self, comment: String) -> Record {
        let mut bytes = comment.as_bytes();
        let mut size = bytes.len();
        let record = Record {
            offset: self.size,
            size: size,
        };
        self.size += size;
        self.file.write_all(bytes);
        record
    }
    //
    //    pub fn read(&mut self, offset: usize, size: usize) {
    //        self.file.read
    //    }
}

fn main() {
    let mut l = Table::new("test.txt".to_string());
    let n = l.write("A☑".to_string());
    println!("Array written is {:?}", n);
}

