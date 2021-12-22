use clap::{App, AppSettings, Arg};

pub fn clap_app() -> App<'static, 'static> {
    App::new("aa2cg")
    .about("All-Atom to Coarse-Grained tool for generating GENESIS CG input\n\
            files from AA structures.")
    .version("0.1.0")
    .setting(AppSettings::ColoredHelp)
    .args(&[
        Arg::with_name("input")
            .value_name("FILE")
            .index(1)
            .required(true)
            .help("Input coordinate file name"),
        Arg::with_name("ff_protein")
            .value_name("NAME")
            .long("--ff-protein")
            .takes_value(true)
            .possible_values(&["AICG2+", "Clementi", "Kb-Go"])
            .default_value("AICG2+")
            .required(false)
            .help("Force field for proteins"),
        Arg::with_name("ff_dna")
            .value_name("NAME")
            .long("--ff-dna")
            .takes_value(true)
            .possible_values(&["3SPN.2C"])
            .default_value("3SPN.2C")
            .required(false)
            .help("Force field for DNA"),
        Arg::with_name("ff_rna")
            .value_name("NAME")
            .long("--ff-rna")
            .takes_value(true)
            .possible_values(&["HT"])
            .default_value("HT")
            .required(false)
            .help("Force field for RNA"),
        Arg::with_name("verbose")
            .long("--verbose")
            .takes_value(false)
            .required(false)
            .help("Verbose mode")
    ])
}
