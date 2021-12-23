use clap::ArgMatches;
use crate::util::file;
use ansi_term::Color;

pub fn app(args: &ArgMatches) {
    let verbose = args.is_present("verbose");

    let input_filename = args.value_of("input").unwrap();
    if verbose {
        println!("{} {}", Color::Blue.paint("Input file: "),
                          Color::Green.paint(input_filename));
    } 

    let extension = file::get_extension_from_filename(input_filename);
    if verbose {
        println!("Input file format: {extension}");
    } 
}
