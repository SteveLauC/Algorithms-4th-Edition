mod sort;

use rand::random;
use sort::{insertion_sort, selection_sort};
use std::{
    env::args,
    process::exit,
    time::{Duration, Instant},
};

fn main() {
    let av: Vec<String> = args().collect();
    if av.len() < 3 {
        eprintln!("usage: {} LEN <ALGORITHM>", av[0]);
        exit(1);
    }

    let len: usize = av[1].parse().unwrap();
    let mut s: Vec<i32> = random_array(len);

    let now: Instant = Instant::now();
    match av[2].as_str() {
        "insertion" => insertion_sort(&mut s),
        "selection" => selection_sort(&mut s),
        _ => todo!("not implemented yet"),
    }
    let elapsed: Duration = now.elapsed();

    println!(
        "Use {} to sort {} numbers, consuming {:?}",
        av[2].as_str(),
        av[1].as_str(),
        elapsed
    );
}

fn random_array(len: usize) -> Vec<i32> {
    (0..len).into_iter().map(|_| random()).collect()
}
