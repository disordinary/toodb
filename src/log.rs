use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::BufReader;
use std::io::Read;
use std::str;
use std::vec;

extern crate serde_json;
use self::serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub enum CRUD {
    create,
    update,
    delete
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    id: String,
    size: usize,
    offset: usize,
    action: CRUD,
}

pub struct Log {
    file: fs::File,
}

impl Log {
    pub fn new(file_name: String) -> Log {
        let mut option = OpenOptions::new();
        option.read(true);
        option.write(false);
        option.append(true);
        option.create(true);
        Log {
            file: option.open(file_name).unwrap(),
        }
    }

    pub fn add(&mut self, id: String, offset: usize, size: usize, action: CRUD) {
        let item = Item {
            id: id,
            size: size,
            offset: offset,
            action: action,
        };
        let mut json = serde_json::to_string(&item).unwrap();
        let toOut = format!("{:0>4}{}", json.len(), json);
        self.file.write_all(toOut.as_bytes());
    }
    pub fn add_new(&mut self, id: String, offset: usize, size: usize) {
        self.add(id, offset, size, CRUD::create)
    }
    pub fn add_update(&mut self, id: String, offset: usize, size: usize) {
        self.add(id, offset, size, CRUD::update)
    }
    pub fn add_delete(&mut self, id: String, offset: usize, size: usize) {
        self.add(id, offset, size, CRUD::delete)
    }

    pub fn read(&mut self) {
        let mut buf = BufReader::new(&mut self.file);
        let mut buffer = vec![0; 4];
        buf.read(&mut buffer);

        let result: &str;

        unsafe {
            result = str::from_utf8_unchecked(&buffer);
        }
        let size = result.parse::<usize>().unwrap();
        let mut buffers = vec![0; size];
        buf.read(&mut buffers);

        let results: &str;

        unsafe {
            results = str::from_utf8_unchecked(&buffers);
        }
        println!("{:?}",results);

       // self._read(3);
//        let my_int = size.parse::<usize>().unwrap();
//
//        println!("{:?}",my_int);
//
//        let x = self._read(2);
//        println!("{:?}",x);
    }

//    pub fn _read>(&mut self, size: usize) -> String {
//        return "TEST".to_string();
//    }
//    fn _read<R: Read>(&self, mut buf: R, size: usize) -> String {
//
//        //let mut buffer: [u8; size] = [0;size];
//        // let mut buffer = Vec::with_capacity(size);
//        let mut buffer = vec![0; size];
//        buf.read(&mut buffer);
//        let result: &str;
//
//        unsafe {
//            result = str::from_utf8_unchecked(&buffer);
//        }
//
//         println!("{:?}",result);
//
//        result.to_string()
//    }

}
