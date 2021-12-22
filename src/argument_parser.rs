use super::aa2cg;
use clap::{App, AppSettings};

pub fn clap_app() -> App<'static, 'static> {
    App::new("cgtools")
        .about("GENESIS Coarse-Grained Tools for generating and manipulating \
                coordinate and\ntopology Molecular Dynamics files.")
        .version("0.1.0")
        .setting(AppSettings::ColoredHelp)
        .subcommand(aa2cg::clap_app())
}
