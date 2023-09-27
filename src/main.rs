use rand::Rng;
use clap::Parser;


#[derive(Parser)]
#[command(author = "pxlmastr", about = "A simple passcode generator for your phone or computer.")]
#[command(version = "1.0")]
struct Args {
    num: i32
}

fn getrandom(num: i32) {
    let mut mystr = String::new();
    for _i in 0..num {
        let num = rand::thread_rng().gen_range(0..9);
        mystr += &num.to_string();
    }
    println!("{}", mystr);
}

fn main() {
    let args = Args::parse();
    getrandom(args.num);
}