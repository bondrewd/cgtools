use clap::{App, AppSettings, Arg};

fn main() {
    let matches = App::new("cgtools")
        .about(
            "GENESIS Coarse-Grained Tools for generating and manipulating coordinate and\ntopology Molecular Dynamics files.",
        )
        .version("0.1.0")
        .setting(AppSettings::ColoredHelp)
        .subcommand(
            App::new("aa2cg")
                .about("All-Atom to Coarse-Grained tool for generating GENESIS CG input\nfiles from AA structures.")
                .version("0.1.0")
                .setting(AppSettings::ColoredHelp)
                .args(&[
                    Arg::with_name("input")
                        .value_name("input")
                        .index(1)
                        .required(true)
                        .help(
                            "Input coordinate file name (Available formats: .pdb, .gro, and .crd)",
                        ),
                    Arg::with_name("output")
                        .value_name("output")
                        .short("-o")
                        .long("--output")
                        .default_value("out")
                        .required(false)
                        .help(
                            "Output coordinate file name (Available formats: .pdb, .gro, and .crd)",
                        ),
                    Arg::with_name("psf")
                        .value_name("psf")
                        .long("--psf")
                        .takes_value(false)
                        .required(false)
                        .help("Generate output topology in PSF format"),
                    Arg::with_name("top")
                        .long("--top")
                        .takes_value(false)
                        .required(false)
                        .help("Generate output topology in TOP format"),
                    Arg::with_name("mmCIF")
                        .long("--mmCIF")
                        .takes_value(false)
                        .required(false)
                        .help("Use the PDBx/mmCIF format for pdb files"),
                ]),
        )
        .get_matches();

    match matches.subcommand() {
        ("aa2cg", Some(sub_matches)) => {
            println!("args: {:?}", sub_matches.value_of("input"))
        }
        _ => unreachable!(),
    };
}
