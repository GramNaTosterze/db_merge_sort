
use std::{env, io::{self, Write}, fs};

use triangle::Triangle;

use natural_sort::tape::Tape;

mod natural_sort;
mod triangle;
mod file_handler;


fn main() -> Result<(), io::Error> {

    io::stdout().flush()?;
    println!("type help for help");
    
    let mut tapes = Vec::new();
    tapes.push(Tape::new());
    loop {

        let mut cmd = String::new();
        if io::stdin().read_line(&mut cmd)? == 0 { break; }
        match cmd.as_str().trim() {
            _ if cmd.starts_with("random") => {
                let len;
                cmd.pop(); //remove '\n'
                let cmd: Vec<&str> = cmd.split(" ").collect();
                if cmd.len() > 1 {
                    len = cmd[1].parse().expect("Wrong argument");
                } else {
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf)?;
                    buf.pop(); //remove '\n'
                    len = buf.parse().expect("wrong argument");
                }
                
                // move to fn but only when everything else works
                for _ in 0..len {
                    let rand_record = rand::random::<Triangle>();
                    tapes[0].push(rand_record);
                }
            }

            _ if cmd.starts_with("add") => {
                let record: Triangle;
                cmd.pop(); //remove '\n'
                let cmd: Vec<&str> = cmd.split(" ").collect();
                if cmd.len() > 1 {
                    record = cmd[1].parse().expect("Wrong format");
                } else {
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf)?;
                    buf.pop(); //remove '\n'
                    record = buf.parse().expect("wrong format");
                }
                tapes[0].push(record);
            } 

            _ if cmd.starts_with("load") => {
                cmd.pop(); //remove '\n'
                let mut path: String = "".to_string();
                let cmd: Vec<&str> = cmd.split(" ").collect();
                if cmd.len() > 1 {
                    path = cmd[1].to_string();
                } else {
                    io::stdin().read_line(&mut path)?;
                }
                
                let mut records_loaded = 0;
                let data  = fs::read_to_string(path)?;
                let records = data.split("\n");
                for record in records {
                    let record: Triangle = record.parse().expect("Wrong format");
                    tapes[0].push(record);
                    records_loaded+=1;
                }
                println!("loaded {records_loaded} records");
            }

            "print enable" => { display_after_run!(true); }
            "print disable" => { display_after_run!(false); }

            "sort" => {
                for tape in &mut tapes {
                    tape.flush()?;
                }
                let info = natural_sort::sort(&mut tapes, 2).expect("Problem sorting");

                println!("end info:");
                println!("number of phases: {}", info.number_of_phases);
                println!("disk operations: {}", info.disk_ops);
                println!("teoretical values: ");
                println!("number of phases: {}", info.teor_number_of_phases);
                println!("disk operations: {}", info.teor_disk_ops);
            }

            "help" => {
                println!("random {{x}} - adds x amount of random records to starting tape");
                println!("add {{record}} - adds record");
                println!("load {{file}} - loads records form file");
                
                println!("print enable/disable - enables/disables printing additional info when sorting");

                println!("sort - sorts given tape");

                println!("clear - flushes stdout");
                println!("exit - exits program");
            }

            "clear" => { io::stdout().flush()?; }
            "exit" => { break }
            _ => { println!("invalid command") }
        }
    }
    Ok(())
}
