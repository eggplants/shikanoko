use rand::seq::SliceRandom;
use std::env;
use std::io::{self, Write};
use std::process;
use std::time::Instant;

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=830afca3cf63b6426b32622107e0e382

fn get_usage() -> String {
    let usage = format!(
        "USAGE: {} <positive int>",
        env::current_exe().unwrap().file_name().unwrap().to_str().unwrap()
    );
    usage
}

fn err_exit(message: String) -> ! {
    eprintln!("{}", get_usage());
    eprintln!("Error: {}", message);
    process::exit(1);
}

fn get_n() -> usize {
    let args: Vec<String> = env::args().skip(1).collect();
    let n: usize = match args.len() {
        0 => {
            println!("{}", get_usage());
            process::exit(0);
        },
        1 => match args[0].parse() {
            Ok(n) => n,
            _ => err_exit("invalid number".to_string()),
        },
        _ => err_exit("invalid arg len".to_string()),
    };
    if n < 1 {
        err_exit(format!("need a positive number, got {}", n));
    };
    n
}

fn main() {
    let n = get_n();

    let mut rng = rand::thread_rng();
    let elements = ["しか", "のこ", "こし", "たん"];
    let target = "しかのこのこのここしたんたん".repeat(n);
    let target_len = target.len();
    let mut output = String::new();

    let start_time = Instant::now();

    loop {
        let c = elements.choose(&mut rng).unwrap();
        print!("{}", c);
        io::stdout().flush().unwrap();

        output.push_str(c);

        if output.contains(&target) {
            let elapsed_time = start_time.elapsed();
            println!("🦌🦌🦌");
            println!("Loop duration: {:.2?}", elapsed_time);
            break;
        }

        if output.len() > target_len {
            output = output.chars().skip(2).collect();
        }
    }
}
