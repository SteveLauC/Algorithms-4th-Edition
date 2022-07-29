mod cli_arg;
mod sort;

use clap::ArgMatches;
use cli_arg::cli_init;
use rand::random;
use sort::{insertion_sort, selection_sort};
use std::time::{Duration, Instant};

fn main() {
    let app: ArgMatches = cli_init();
    let len: usize = app.value_of("n").unwrap().parse().unwrap();
    let use_ordered_data: bool = app.is_present("ordered-data");
    let mut s: Vec<usize> = random_array(len, use_ordered_data);
    let algorithm: &str = app.value_of("algorithm").unwrap();

    let now: Instant = Instant::now();
    match algorithm {
        "selection" => selection_sort(&mut s),
        "insertion" => insertion_sort(&mut s),
        _ => todo!("not implemented yet"),
    }
    let elapsed: Duration = now.elapsed();

    println!(
        "Use <{} sort> to sort {} {} numbers, consuming {:?}",
        algorithm,
        len,
        if use_ordered_data {
            "ordered"
        } else {
            "random"
        },
        elapsed
    );
}

fn random_array(len: usize, use_ordered_data: bool) -> Vec<usize> {
    match use_ordered_data {
        true => (0..len).collect(),
        false => (0..len).map(|_| random()).collect(),
    }
}
