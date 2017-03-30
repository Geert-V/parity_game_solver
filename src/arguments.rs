use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::process;
use std::fmt;


#[derive(Debug, Copy, Clone)]
pub enum StrategySort {
    Random,
    Input,
    Priority,
    Succesor,
    SelfLoop
}

impl fmt::Display for StrategySort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // write!(f, "{}", match self {
        //     Random => "Random",
        //     Input => "Input",
        //     Priority => "Priority",
        //     Succesor => "Succesor",
        //     SelfLoop => "SelfLoop"
        // })
    }
}

#[derive(Debug, Clone)]
pub struct Arguments {
    pub testing: bool,
    pub pg_file: String,
    pub strategy: Option<StrategySort>
}

fn exit_and_print_usage(args: &Vec<String>) {
    let prog_name = Path::new(&args[0])
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or("<application name>");

    println!("usage: {} -pg <file path> [-input]/[-random]/[-priority]/[-selfloop]/[-succesor]", prog_name);
    println!("or: {} [-ex <directory path>]", prog_name);
    process::exit(0);
}

pub fn get() -> Arguments {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 3 {
        exit_and_print_usage(&args);
    }

    let mut pg_file = None;
    let mut strategy = None;

    let mut args_iter = args.iter();

    // Skip the first argument as this is the program name.
    args_iter.next();
    let mut testing = false;

    loop {
        let arg = args_iter.next();
        
        if arg.is_none() {
            break;
        }

        match arg.unwrap().to_lowercase().as_ref() {
            "-ex" => {
                testing = true;
                pg_file = args_iter.next();

                if pg_file.is_none() {
                    exit_and_print_usage(&args);
                }
                strategy = None;
            },
            "-pg" => {
                pg_file = args_iter.next();

                if pg_file.is_none() {
                    exit_and_print_usage(&args);
                }
            },
            "-input" => {
                if strategy.is_some() {
                    exit_and_print_usage(&args);
                }

                strategy = Some(StrategySort::Input);
            }
            "-random" => {
                if strategy.is_some() {
                    exit_and_print_usage(&args);
                }

                strategy = Some(StrategySort::Random);
            },
            "-priority" => {
                if strategy.is_some() {
                    exit_and_print_usage(&args);
                }

                strategy = Some(StrategySort::Priority);
            },
            "-succesor" => {
                if strategy.is_some() {
                    exit_and_print_usage(&args);
                }

                strategy = Some(StrategySort::Succesor);
            },
            "-selfloop" => {
                if strategy.is_some() {
                    exit_and_print_usage(&args);
                }

                strategy = Some(StrategySort::SelfLoop);
            },
            x => {
                println!("Unknown: {}", x);
                exit_and_print_usage(&args);
            },
        };
    }

    if pg_file.is_none() || (strategy.is_none() && !testing) {
        exit_and_print_usage(&args);
    }

    Arguments {
        pg_file: pg_file.unwrap().clone(),
        testing: testing,
        strategy: strategy
    }
}