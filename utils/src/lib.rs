use clap::{App, Arg};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_input(app_name: &str) -> String {
    App::new(app_name)
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches()
        .value_of("INPUT")
        .unwrap()
        .into()
}

pub fn read_lines(filename: &str) -> std::io::Lines<BufReader<File>> {
    let file = File::open(filename).expect("Couldn't open input file.");
    BufReader::new(file).lines()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
