use clap::ArgMatches;
use crate::util::file;

pub fn app(args: &ArgMatches) {
    let verbose_lvl = args.value_of("verbosity")
                        .unwrap()
                        .to_string()
                        .parse::<u8>()
                        .unwrap();

    let input_filename = args.value_of("input").unwrap();
    if verbose_lvl > 1 {
        println!("Input file: {input_filename}")
    } 

    let extension = file::get_extension_from_filename(input_filename);
    if verbose_lvl > 1 {
        println!("Input file format: {extension}")
    } 
}
