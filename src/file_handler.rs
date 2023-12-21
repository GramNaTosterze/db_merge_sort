use std::{
    mem::size_of,
    path::PathBuf,
    io::{Write, Read, Error, Seek},
    ptr,
    io::ErrorKind,
    fs::File
};
use serde::{Serialize, Deserialize};

use crate::triangle::Triangle;

pub const BLOCK_SIZE: usize = 1000*size_of::<Triangle>();

pub struct FileHandler {
    /* File */
    path: PathBuf,
    file: File,
    end_of_file: bool,

    /* data Block */
    page: Vec<u8>,
    disk_ops: usize
}

impl FileHandler {
    /* public methods */
    pub fn open(path: PathBuf) -> Result<Self, Error> {
        return match File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path.clone()) {
            Ok(file) => {
                Ok(Self {
                    path,
                    file,
                    end_of_file: false,
                    page: Vec::with_capacity(BLOCK_SIZE),
                    disk_ops: 0
                })
            }
            Err(err) => Err(err)
        }
    }

    pub fn view<T>(&mut self) -> Result<T, bincode::Error>
    where T: for<'a> Deserialize<'a> {
        if self.page.len() == 0 { self.read_page()?; }
        let page = &self.page[0..size_of::<T>()];
        let obj: T = bincode::deserialize::<T>(&page)?;
        
        if self.page.len() == 0 { self.read_page()?; }
        Ok(obj)
    }
    pub fn read<T>(&mut self) -> Result<T, bincode::Error> 
    where T: for<'a> Deserialize<'a> {
        if self.page.len() == 0 { self.read_page()?; }
        
        if self.page.len() < size_of::<T>() { return Err(Error::new(ErrorKind::Other, "page empty").into()); }
        let mut page = self.page.split_off(size_of::<T>());
        unsafe {ptr::swap(&mut page, &mut self.page);}
        let obj: T = bincode::deserialize::<T>(&page)?;

        if self.page.len() == 0 { self.read_page()?; }
        Ok(obj)
    }
    pub fn write<T>(&mut self, obj: T) -> Result<(), bincode::Error> 
    where T: Serialize {
        if self.page.len() == BLOCK_SIZE { self.write_page()?; }

        let mut page = bincode::serialize(&obj)?;
        self.page.append(&mut page);
        Ok(())
    }
    pub fn flush(&mut self) -> Result<(),Error> {
        self.write_page()?;
        self.rewind()?;
        Ok(())
    }
    pub fn clear(&mut self) -> Result<(), Error> {
        self.rewind()?;
        self.file.set_len(0)?;
        self.disk_ops = 0;
        Ok(())
    }
    pub fn rewind(&mut self) -> Result<(), Error> {
        self.end_of_file = false;
        self.file.rewind()?;
        self.page.clear();
        Ok(())
    }
    pub fn print_content<T>(&mut self) -> Result<(), Error> 
    where T: std::fmt::Display+for<'a> Deserialize<'a> {

        let disk_ops = self.disk_ops;

        let page = self.page.clone();
        self.rewind()?;
        //self.page = page.clone();

        while !self.eof() || self.page.len() != 0 {
            match self.read::<T>() {
                Ok(obj) => println!("{obj}"),
                Err(_) => break
            }
        }
        if page.len() >= size_of::<T>() {
            for i in (0..(page.len())).step_by(size_of::<T>()) {
                let obj = bincode::deserialize::<T>(&page[i..i+size_of::<T>()]).unwrap();
                println!("{obj}")
            }
        }

        self.rewind()?;
        self.page = page.clone();
        self.disk_ops = disk_ops;
        Ok(())
    }
    pub fn eof(&self) -> bool { return self.end_of_file && self.page.len() == 0; }
    /* getters */
    pub fn path(&self) -> PathBuf { return self.path.clone(); }
    pub fn disk_ops(&self) -> usize { return self.disk_ops; }


    /* private methods */

    fn read_page(&mut self) -> Result<(), Error> {
        if self.end_of_file { return Ok((/* no more blocks */)) }
        if self.page.len() != 0 { return Err(Error::new(ErrorKind::Other, "block not empty")); }

        self.page = vec![0; BLOCK_SIZE];
        let bytes = self.file.read(&mut self.page)?;
        
        if bytes < BLOCK_SIZE {
            self.page = self.page[0..bytes].to_vec();
            self.end_of_file = true;
        }
        
        self.disk_ops+=1;
        Ok(())
    }
    fn write_page(&mut self) -> Result<(),Error> {
        if self.page.len() == 0 { return Ok(()) }

        self.file.write(&self.page)?;
        self.page.clear();

        if self.end_of_file { self.end_of_file=false; }
        self.disk_ops+=1;
        Ok(())
    }
}