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

const BLOCK_SIZE: usize = 10*size_of::<Triangle>();

pub struct FileHandler {
    /* File */
    path: PathBuf,
    file: File,
    end_of_file: bool,

    /* data Block */
    data: Vec<u8>
}

impl FileHandler {
    /* public methods */
    pub fn create(path: PathBuf) -> Result<Self, Error> {
        return match File::create(path.clone()) {
            Ok(file) => {
                Ok(Self {
                    path,
                    file,
                    end_of_file: false,
                    data: Vec::with_capacity(BLOCK_SIZE)
                })
            }
            Err(err) => Err(err)
        }
    }
    pub fn open(path: PathBuf) -> Result<Self, Error> {
        return match File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(path.clone()) {
            Ok(file) => {
                Ok(Self {
                    path,
                    file,
                    end_of_file: false,
                    data: Vec::with_capacity(BLOCK_SIZE)
                })
            }
            Err(err) => Err(err)
        }
    }

    pub fn view<T>(&mut self) -> Result<T, bincode::Error>
    where T: for<'a> Deserialize<'a> {
        if self.data.len() == 0 {
            self.read_block()?;
        }

        let data = &self.data[0..size_of::<T>()];
        let obj: T = bincode::deserialize::<T>(&data)?;
        
        if self.data.len() == 0 {
            self.read_block()?;
        }
        Ok(obj)
    }
    pub fn read<T>(&mut self) -> Result<T, bincode::Error> 
    where T: for<'a> Deserialize<'a> {
        if self.data.len() == 0 {
            self.read_block()?;
        }

        let mut data = self.data.split_off(size_of::<T>());
        unsafe {ptr::swap(&mut data, &mut self.data);}
        let obj: T = bincode::deserialize::<T>(&data)?;

        if self.data.len() == 0 {
            self.read_block()?;
        }
        Ok(obj)
    }
    pub fn write<T>(&mut self, obj: T) -> Result<(), bincode::Error> 
    where T: Serialize {
        if self.data.len() == BLOCK_SIZE {
            self.write_block()?;
        }

        let mut data = bincode::serialize(&obj)?;
        //assert!(data.len() + self.data.len() <= self.data.capacity());
        self.data.append(&mut data);
        Ok(())
    }
    pub fn flush(&mut self) -> Result<(),Error> {
        self.write_block()?;
        self.data.clear();
        Ok(())
    }
    pub fn clear(&mut self) -> Result<(), Error> {
        self.file.rewind()?;
        self.file.set_len(0)?;
        self.data.clear();
        Ok(())
    }
    pub fn print_content<T>(&mut self) -> Result<(), Error> 
    where T: std::fmt::Display+for<'a> Deserialize<'a> {
        let mut number_of_objs = 0;
        /* save current state */
        let position = self.file.stream_position()?;
        let data = self.data.clone();
        let end_of_file = self.end_of_file;

        self.end_of_file = false;
        self.file.rewind()?;
        self.data.clear();
        while !self.eof() || self.data.len() != 0 {
            println!("{}", self.read::<T>().expect("Problem deserializing"));
            number_of_objs+=1;
        }

        /* restore current state */
        //self.file.seek(SeekFrom::Start(position))?;
        //self.data = data;
        //self.end_of_file = end_of_file;
        self.end_of_file = false;
        self.file.rewind()?;
        self.data.clear();

        println!("N: {number_of_objs}");
        Ok(())
    }
    /* getters */
    pub fn eof(&self) -> bool {
        return self.end_of_file;
    }
    pub fn path(&self) -> PathBuf {
        return self.path.clone();
    }


    /* private methods */

    fn read_block(&mut self) -> Result<(), Error> {
        if self.end_of_file { return Ok((/* no more blocks */)) }
        if self.data.len() != 0 { return Err(Error::new(ErrorKind::Other, "block not empty")); }

        self.data = vec![0; 240];
        let bytes = self.file.read(&mut self.data)?;
        if bytes < BLOCK_SIZE {
            self.data = self.data[0..bytes].to_vec();
            self.end_of_file = true;
        }
        Ok(())
    }
    fn write_block(&mut self) -> Result<(),Error> {
        if self.data.len() == 0 {
            return Err(Error::new(ErrorKind::Other, "block empty"));
        }
        self.file.write(&self.data)?;
        self.data.clear();
        Ok(())
    }
}