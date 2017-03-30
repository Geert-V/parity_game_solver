mod arguments;
mod pg;
mod parser;
mod algorithm;
mod strategies;

use arguments::*;
use pg::*;
use strategies::*;
use std::thread;
use std::fs;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::cmp::Ordering;
use std::path::PathBuf;
use std::usize;
use std::str::FromStr;
use std::time::{Duration, Instant};
use std::os::unix::thread::JoinHandleExt;


extern crate regex;
use self::regex::Regex;

use std::cmp;
extern crate libc;



fn compare(a: &ExperimentFile, b: &ExperimentFile) -> Ordering {
    let re: Regex = Regex::new(r"[_.]").unwrap();
    let re2: Regex = Regex::new(r"^[0-9]+$").unwrap();
    let fields1: Vec<&str> = re.split(a.path.as_str()).collect();
    let fields2: Vec<&str> = re.split(b.path.as_str()).collect();

    let mut ordering = Ordering::Equal;
    for i in 0..cmp::min(fields1.len(), fields2.len()) {
        let f1 = fields1.get(i).unwrap();
        let f2 = fields2.get(i).unwrap();
        if re2.is_match(f1) && re2.is_match(f2) {
            ordering = usize::from_str(f1).unwrap_or(0).cmp(&usize::from_str(f2).unwrap_or(0));
        } else {
            ordering = f1.cmp(f2);
        }
        if ordering != Ordering::Equal {
            break;
        }
    }
    // let convert = lambda text: int(text) if text.isdigit() else text.lower() 
    // let alphanum_key = lambda key: [ convert(c) for c in re.split('([0-9]+)', key[0]) ] 
    // let paths = sorted(paths, key = alphanum_key)

    return ordering;
}

struct ExperimentFile {
    path: String,
    file_name: String
}

fn parse<'exp>(x: PathBuf) -> ExperimentFile {
    ExperimentFile { 
        path: x.clone().into_os_string().into_string().unwrap(),
        file_name: x.file_name().unwrap().to_os_string().into_string().unwrap()
    }
}

fn main() {
    println!("");
    let args = arguments::get();
    if !args.testing {
        let game = parser::parse_from_file(&args.pg_file);
        println!("");
        println!("Maximal measure: {:?}", game.max_measure());
    
        match args.strategy.unwrap() {
            StrategySort::Random => run(&game, &RandomStrategy::new(&game)),
            StrategySort::Input => run(&game, &InputStrategy::new(&game)),
            StrategySort::Priority => run(&game, &PriorityStrategy::new(&game)),
            StrategySort::Succesor => run(&game, &SuccesorStrategy::new(&game)),
            StrategySort::SelfLoop => run(&game, &SelfLoopStrategy::new(&game))
        };
    } else {
        let dir_name = args.pg_file.clone();
        let paths = fs::read_dir(dir_name).unwrap();
        let mut strategies = vec!(StrategySort::Random, StrategySort::Input, StrategySort::Priority, StrategySort::Succesor, StrategySort::SelfLoop);
    
        let l = paths.collect::<Vec<_>>().into_iter().map(|x| x.unwrap().path());
        let mut l: Vec<ExperimentFile> = l.into_iter().map(|x| parse(x)).collect();
        l.sort_by(|a, b| compare(&a, &b));
        for path in l {
            let (tx, rx) = mpsc::channel();
            // let path = path;//.unwrap().path().clone();
            println!("Name: {}", path.file_name);
            let data = Arc::new(path);
            let mut threads = Vec::new();
            for strat in strategies.clone() {
                let (data, tx) = (data.clone(), tx.clone());
                threads.push(thread::spawn(move || {
                    let file = data;
                    let game = parser::parse_from_file(&file.path);
                    let p = match strat {
                        StrategySort::Random => run(&game, &RandomStrategy::new(&game)),
                        StrategySort::Input => run(&game, &InputStrategy::new(&game)),
                        StrategySort::Priority => run(&game, &PriorityStrategy::new(&game)),
                        StrategySort::Succesor => run(&game, &SuccesorStrategy::new(&game)),
                        StrategySort::SelfLoop => run(&game, &SelfLoopStrategy::new(&game))
                    };
                    // file_name
                    // strategy
                    // node 0
                    // iterations
                    let c = p.prog.winning_set(Owner::Odd).contains(&0);
                    println!("{},{},{},{}", file.file_name, strat, if c { "odd" } else { "even" }, p.nr_of_iterations);
                    tx.send(strat).unwrap();
                }));
            }
            let start = Instant::now();
            let timeout = Duration::from_millis(1000 * 2 * 60);
            let mut surviving_strategies = vec!();
            for _ in strategies.clone() {
                let diff = Instant::now()+ Duration::from_millis(10) - start;
                if timeout + Duration::from_millis(600) > diff {
                    match rx.recv_timeout(timeout + Duration::from_millis(100) - diff) {
                        Ok(s) => { surviving_strategies.push(s); }
                        _ => {}
                    }
                }
                // match x {
                    
                // }
            }
            strategies = surviving_strategies;
            for t in threads {
                let pt = t.into_pthread_t();
                unsafe {
                    libc::kill(pt as i32, 15);
                }
            }
        }
    }
}

fn run(game: &Game, strat: &Strategy) -> algorithm::SpmResult {
    algorithm::small_progress_measures(&game, strat)
    // println!("");
    // println!("Won even: {:?}", progress.winning_set(Owner::Even));
    // println!("");
    // println!("Won odd : {:?}", progress.winning_set(Owner::Odd));
}