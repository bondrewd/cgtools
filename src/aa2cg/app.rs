use std::fs;
use std::path;

use clap::ArgMatches;
use colored::Colorize;

use crate::molecule;
use crate::util::mdfile;

pub fn app(args: &ArgMatches) {
    let verbose = args.is_present("verbose");

    if verbose {
        println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
        println!("┃{:^58}┃", "AA2CG".bold());
        println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
        println!("{}", "Input file information".bold());
    }

    let input_path = args.value_of("input").unwrap();
    let input_path = path::Path::new(input_path);
    let input_filename = input_path.file_name()
                                   .unwrap()
                                   .to_str()
                                   .unwrap();
    if verbose {
        println!("├─ {} {}", "Input file:".blue().bold(), input_filename.green());
    } 

    let extension = mdfile::get_extension_from_filename(input_filename);
    if verbose {
        println!("├─ {} {}", "Input format:".blue().bold(), extension.to_string().green());
    } 

    let mut file = match fs::File::open(input_path) {
        Ok(f) => f,
        Err(e) => {
            if verbose {
                println!("└─ {} {} {}", "Error:".red().bold(), "Can not open input file.", e);
            } else {
                println!("{} {} {}", "Error:".red().bold(), "Can not open input file.", e);
            }
            return;
        },
    };

    let structure = match extension {
        mdfile::FileExtension::Pdb => molecule::Structure::from_pdb_file(file, verbose),
        _ => molecule::Structure::from_pdb_file(file, verbose),
    };
    if verbose {
        println!("└──┬─ {} {}", "Atoms:".yellow().bold(), structure.atom_name.len());
        println!("   ├─ {} {}", "Residues:".yellow().bold(), structure.residue_name.len());
        println!("   └─ {} {}", "Chains:".yellow().bold(), structure.chain_name.len());
    }
}
