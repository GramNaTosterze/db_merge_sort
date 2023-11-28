/* Big buffer sort */

use std::{
    io::Error,
    cmp::PartialOrd,
    marker::Copy,
    path::PathBuf,
    fmt::Display
};
use rand::distributions::{Distribution, Standard};
use serde::{Serialize, Deserialize};
use cute::c;

use crate::file_handler::FileHandler;

use self::tape::Tape;

pub mod tape;

pub fn random_data<R>(len: usize) -> Result<String, Error>
where R: Serialize+for<'a> Deserialize<'a>+PartialOrd+Display+Copy, Standard: Distribution<R> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("data/data.hex"));
    match FileHandler::create(path.clone()) {
        Ok(mut file) => {
            for _ in 0..len {
                let rand_record = rand::random::<R>();
                file.write(rand_record).expect("Problem writing to file");
            }
            file.flush()?;
        }
        Err(err) => return Err(err)
    }
    
    Ok(path.into_os_string().into_string().unwrap())
}

pub fn sort<R>(path: String, n: usize) -> Result<(), Error>
where R: Serialize+for<'a> Deserialize<'a>+PartialOrd+Display+Copy, Standard: Distribution<R> {
    let mut file = FileHandler::open(path.clone().into())?;
    file.print_content::<R>()?;

    let mut target: Vec<Tape<R>> = Vec::new();
    target.push(Tape::from_file(path.clone()));

    let mut tapes: Vec<Tape<R>> = c![Tape::new(), for _i in 0..n];
    while !is_sorted(&mut target) {
        distribute(&mut target[0], &mut tapes);
        clear_tapes(&mut target);

        print_info(&mut tapes);
        println!();println!();

        merge(&mut tapes, &mut target);
        print_info(&mut target);
        println!();println!();
        clear_tapes(&mut tapes);
    }

    Ok(())
}

fn print_info<R>(tapes: &mut Vec<Tape<R>>)
where R: Serialize+for<'a> Deserialize<'a>+PartialOrd+Display+Copy, Standard: Distribution<R> {
    let mut tape_num = 0;
    for i in 0..tapes.len() {
        println!("t{tape_num}");
        tape_num+=1;
        tapes[i].print();
    }
}

fn distribute<R>(source: &mut Tape<R>, target: &mut Vec<Tape<R>>)
where R: Serialize+for<'a> Deserialize<'a>+PartialOrd+Display+Copy, Standard: Distribution<R> {
    let mut i = 0;
    let mut number_of_runs = 1;
    let mut last_record: Option<R> = None;
    let mut n = 0;
    while !source.is_empty() {
        let record: R = source.next_record();
        if last_record == None {
            last_record = Some(record);
        } else if Some(record) <= last_record {
            number_of_runs+=1;
            target[i].run_len.push(n);
            i = (i+1)%(target.len()); /* cycle through targets */
            n = 0;
        }
        target[i].push(record);
        last_record = Some(record);
        n+=1;
        
    }
    target[i].run_len.push(n);

    for i in 0..target.len() {
        target[i].flush().expect("TODO: panic message");
    }
    println!("number of runs: {number_of_runs}");
}


fn merge<R>(tapes: &mut Vec<Tape<R>>, target: &mut Vec<Tape<R>>)
where R: Serialize+for<'a> Deserialize<'a>+PartialOrd+Display, Standard: Distribution<R> {
    /* from source tapes to target tapes */
    let mut target_idx = 0;
    for run in 0..3 { //target.len()
        let mut idx: Vec<usize> = vec![0; tapes.len()];
        loop {
            let mut min_record: Option<R> = None;
            let mut tape_idx = 0;
            for i in 0..tapes.len() {
                if tapes[i].run_len.len() <= run || idx[i] >= tapes[i].run_len[run] { continue; }
                let obj = tapes[i].view_record();
                if min_record == None || Some(obj) <= min_record {
                    min_record = Some(tapes[i].view_record());
                    tape_idx = i;
                }
            }
            if min_record == None { break; }
            target[target_idx].push(tapes[tape_idx].next_record());
            idx[tape_idx] += 1;
        }
        if idx.iter().sum::<usize>() != 0 {target[target_idx].run_len.push(idx.iter().sum());}
        target_idx = (target_idx+1)%(target.len());
    }
    for i in 0..target.len() {
        target[i].flush().expect("Problem flushing tape");
    }
}

fn is_sorted<R>(tapes: &mut Vec<Tape<R>>)-> bool 
where R: Serialize+for<'a> Deserialize<'a>+PartialOrd+Display, Standard: Distribution<R> {
    for tape in tapes {
        if tape.run_len.len() != 1 {return false;} /* if only one run remains it means it is sorted */
    }
    return true;
}

fn clear_tapes<R>(tapes: &mut Vec<Tape<R>>)
where R: Serialize+for<'a> Deserialize<'a>+PartialOrd+Display, Standard: Distribution<R> {
    for i in 0..tapes.len() {
        tapes[i].clear().expect("TODO: panic message");
    }
}
