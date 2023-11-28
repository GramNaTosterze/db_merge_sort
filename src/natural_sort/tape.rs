//use triangle::Record;
use core::panic;

use std::{
    sync::Mutex,
    marker::PhantomData,
    fs,
    path::PathBuf,
    fmt::Display,
    io
};
use rand::prelude::Distribution;
use rand::distributions::Standard;
use serde::{Serialize, Deserialize};

use crate::file_handler::FileHandler;
static TAPE_NUM: Mutex<usize> = Mutex::new(0);

pub struct Tape<R> 
where R: Serialize+for<'a> Deserialize<'a>+PartialOrd+Display, Standard: Distribution<R> {
    record_type: PhantomData<R>,
    pub idx: usize,
    pub run_len: Vec<usize>,
    file: FileHandler,
}

impl<R> Tape<R> 
where R: Serialize+for<'a> Deserialize<'a>+PartialOrd+Display, Standard: Distribution<R> {
    /* public methods */
    pub fn new() -> Self {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut tape_num = TAPE_NUM.lock().unwrap(); 
        path.push(format!("data/t{tape_num}.tape"));

        match FileHandler::open(path) {
            Ok(file) => {
                *tape_num+=1;
                return Self{
                    record_type: PhantomData,
                    idx:0,
                    run_len:Vec::new(),
                    file, 
                }
            }
            Err(_) => { panic!("Could not create file"); }
        }
    }
    pub fn from_file(path_buf: String) -> Self {
        let tape: Tape<R> = Tape::new();
        fs::copy(path_buf, tape.file.path()).expect("TODO: panic message");
        return tape;
    }

    pub fn next_record(&mut self) -> R {
        match self.file.read::<R>() {
            Ok(record) => return record,
            Err(_) => panic!("Problem deserializing record")
        }
    }
    pub fn view_record(&mut self) -> R {
        match self.file.view::<R>() {
            Ok(record) => return record,
            Err(_) => panic!("Problem deserializing record")
        }
    }
    pub fn push(&mut self, record: R) {
        self.file.write(record).expect("Problem serializing record");
    }
    pub fn is_empty(&mut self) -> bool {
        return self.file.eof();
    }
    pub fn print(&mut self) {
        self.file.print_content::<R>().expect("TODO: panic message");
        print!("n: ");
        for run in &mut *self.run_len { print!("{run} "); }
        println!();
    }
    pub fn flush(&mut self) -> Result<(), io::Error> {
        self.file.flush()
    }
    pub fn clear(&mut self) -> Result<(), io::Error> {
        self.file.clear()?;
        self.run_len = Vec::new();
        Ok(())
    }
}

impl<R> Drop for Tape<R> 
where R: Serialize+for<'a> Deserialize<'a>+PartialOrd+Display, Standard: Distribution<R> {
    fn drop(&mut self) {
        /* remove created tape */
        fs::remove_file(&self.file.path()).unwrap_or_else(|err| {panic!("Cannot remove file: {err}")});
    }
}