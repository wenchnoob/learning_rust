use std::io;
use std::usize;
use std::convert::TryFrom;

fn main() {
    println!("What value index of the fib seq do you want?: ");
    let mut memo: [u32; 1000] = [0; 1000];
    println!("{}", fib(read_value_of_n(), &mut memo));
}

fn fib(n: u32, memo: &mut [u32; 1000]) -> u32 {
    let n_us = usize::try_from(n).unwrap();

    if n < 2 {
        return 1;
    }

    if memo[n_us] != 0 {
        return memo[n_us];
    }

    let v = fib(n-2, memo) + fib(n-1, memo);
    memo[n_us] = v;
    return v;
}

fn read_value_of_n() -> u32 {
    let mut input = String::new();

    let _ = match io::stdin().read_line(&mut input) {
        Ok(size) => size,
        Err(_) => {
            println!("Failed to fetch user input");
            std::process::exit(1);
        }
    };

    return match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Failed to fetch user input");
            std::process::exit(1);
        }
    };
}
