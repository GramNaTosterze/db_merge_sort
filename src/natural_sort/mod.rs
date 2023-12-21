use std::{
    io::Error,
    cmp::{PartialOrd, max},
    marker::Copy,
    fmt::Display,
    sync::Mutex, str::FromStr, mem::size_of
};
use rand::distributions::{Distribution, Standard};
use serde::{Serialize, Deserialize};
use cute::c;


use crate::file_handler::BLOCK_SIZE;

use self::tape::Tape;

pub mod tape;

pub static DISPLAY_AFTER_RUN: Mutex<bool> = Mutex::new(true);

#[macro_export]
macro_rules! display_after_run {
    ($b: expr) => {
        let mut display = natural_sort::DISPLAY_AFTER_RUN.lock().unwrap();
        *display = $b;
    };
    () => {
        let mut display = natural_sort::DISPLAY_AFTER_RUN.lock().unwrap();
        *display = true; 
    };
}

pub struct SortInfo {
    pub number_of_phases: usize,
    pub disk_ops: usize,

    pub teor_number_of_phases: f32,
    pub teor_disk_ops: f32

}


pub fn sort<R>(target: &mut Vec<Tape<R>>, n: usize) -> Result<SortInfo, Error>
where R: Serialize+for<'a> Deserialize<'a>+PartialOrd+Display+Copy+FromStr, Standard: Distribution<R> {
    let mut disk_ops = 0;
    let mut number_of_phases = 0;
    let mut initial_runs = 1;
    let mut record_num = 1;
    let mut assigned = false;
    target[0].print();

    let mut tapes: Vec<Tape<R>> = c![Tape::new(), for _i in 0..n];
    while !is_sorted(target) {
        if !assigned {
            (initial_runs, record_num) = distribute(&mut target[0], &mut tapes);
            assigned = true;
        } else {
            distribute(&mut target[0], &mut tapes);
        }
        for i in 0..target.len() { disk_ops+=target[i].disk_ops() }
        clear_tapes(target);


        let display = DISPLAY_AFTER_RUN.lock().unwrap();
        if *display {
            print_info(&mut tapes);
            println!();println!();
        }
        merge(&mut tapes, target);
        if *display {
            print_info(target);
        }

        for i in 0..tapes.len() { disk_ops+=tapes[i].disk_ops() }
        
        clear_tapes(&mut tapes);
        number_of_phases+=1;
    }
    println!();println!();
    target[0].print();


    let b: f32 = (BLOCK_SIZE/size_of::<R>()) as f32;
    return Ok(SortInfo {
        number_of_phases,
        disk_ops,

        teor_number_of_phases: (initial_runs as f32).log2().ceil(),
        teor_disk_ops: 4_f32*(record_num as f32)*((initial_runs as f32).log2().ceil())/b
    })
}

fn print_info<R>(tapes: &mut Vec<Tape<R>>)
where R: Serialize+for<'a> Deserialize<'a>+PartialOrd+Display+Copy+FromStr, Standard: Distribution<R> {
    let mut tape_num = 0;
    for i in 0..tapes.len() {
        println!("t{tape_num}");
        tape_num+=1;
        tapes[i].print();
    }
}

fn distribute<R>(source: &mut Tape<R>, target: &mut Vec<Tape<R>>) -> (usize, usize)
where R: Serialize+for<'a> Deserialize<'a>+PartialOrd+Display+Copy+FromStr, Standard: Distribution<R> {
    let mut i = 0;
    let mut number_of_runs = 1;
    let mut last_record: Option<R> = None;
    let mut run = 0;
    let mut n = 0;
    
    while !source.is_empty() {
        let record: R = source.next_record();
        if Some(record) < last_record {
            number_of_runs+=1;
            target[i].run_len.push(run);
            i = (i+1)%(target.len()); /* cycle through targets */
            run = 0;
        }
        target[i].push(record);
        last_record = Some(record);
        run+=1;
        n+=1;
    }
    target[i].run_len.push(run);

    for tape in target {
        tape.flush().expect("cannot flush tape");
    }
    return (number_of_runs,n);
}


fn merge<R>(tapes: &mut Vec<Tape<R>>, target: &mut Vec<Tape<R>>)
where R: Serialize+for<'a> Deserialize<'a>+PartialOrd+Display+FromStr+Copy, Standard: Distribution<R> {
    /* from source tapes to target tapes */
    let mut target_idx = 0;
    let runs = max_run(tapes);
    for run in 0..runs {
        let mut idx: Vec<usize> = vec![0; tapes.len()];
        loop {
            let mut min_record: Option<R> = None;
            let mut tape_idx = 0;
            for i in 0..tapes.len() {
                if tapes[i].run_len.len() <= run || idx[i] >= tapes[i].run_len[run] { continue; }
                let obj = tapes[i].view_record();
                if min_record == None || Some(obj) <= min_record {
                    min_record = Some(obj);
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
    for tape in target {
        tape.flush().expect("cannot flush tape");
    }

}

fn is_sorted<R>(tapes: &mut Vec<Tape<R>>)-> bool 
where R: Serialize+for<'a> Deserialize<'a>+PartialOrd+Display+FromStr, Standard: Distribution<R> {
    for tape in tapes {
        if tape.run_len.len() != 1 {return false;} /* if only one run remains it means it is sorted */
    }
    return true;
}

fn clear_tapes<R>(tapes: &mut Vec<Tape<R>>)
where R: Serialize+for<'a> Deserialize<'a>+PartialOrd+Display+FromStr, Standard: Distribution<R> {
    for i in 0..tapes.len() {
        tapes[i].clear().expect("TODO: panic message");
    }
}

fn max_run<R>(tapes: &mut Vec<Tape<R>>) -> usize
where R: Serialize+for<'a> Deserialize<'a>+PartialOrd+Display+FromStr, Standard: Distribution<R> {
    let mut max_run = None;
    for i in 0..tapes.len() {
        max_run = max(max_run, Some(tapes[i].run_len.len()));
    }
    return if let Some(ret) = max_run {ret.clone()} else {0};
}