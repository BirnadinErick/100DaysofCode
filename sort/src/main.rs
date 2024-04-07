use clap::Parser;
use sort::{bubble_sort, quick_sort};

#[derive(Parser)]
#[command(version)]
struct CLIAPI {
    #[arg(short, long, action=clap::ArgAction::Count)]
    switch: u8,
    nums: Vec<i32>,
}

fn main() {
    let args = CLIAPI::parse();

    let mut nums = args.nums;

    if args.switch % 2 == 0 {
        quick_sort(&mut nums);
    } else {
        bubble_sort(&mut nums);
    }

    println!("{:?}", nums);
}
