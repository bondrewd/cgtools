mod constants;
mod molecule;
mod util;

mod aa2cg;

mod argument_parser;
use argument_parser::clap_app;

fn main() {
    let matches = clap_app().get_matches();

    match matches.subcommand() {
        ("aa2cg", Some(sub_matches)) => aa2cg::app(sub_matches),
        _ => unreachable!(),
    };
}
