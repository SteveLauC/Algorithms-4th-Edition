use clap::{Arg, ArgMatches, Command};

pub fn cli_init() -> ArgMatches {
    Command::new("mst")
        .arg(
            Arg::new("ordered-data")
                .takes_value(false)
                .short('o')
                .help("use ordered input data instead of the random one"),
        )
        .arg(
            Arg::new("n")
                .takes_value(true)
                .required(true)
                .help("how many items you wanna sort"),
        )
        .arg(
            Arg::new("algorithm")
                .takes_value(true)
                .required(true)
                .help("which algorithm you wanna use"),
        )
        .get_matches()
}
