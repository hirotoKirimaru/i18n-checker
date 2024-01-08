use clap::{arg, command, value_parser, ArgAction, Command};
use i18n_checker::diff::diff;
use std::path::PathBuf;
// use crate::lib::diff;
// use crate::diff::diff;
// use crate::lib::export;

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .subcommand(
            Command::new("diff")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("diff") {
        println!("DIFF Command received!");
        diff("tests/resources/diff/03");
        // "$ myapp test" was run
        // if matches.get_flag("list") {
        //     // "$ myapp test -l" was run
        //     println!("Printing testing lists...");
        // } else {
        //     println!("Not printing testing lists...");
        // }
    }

    if let Some(matches) = matches.subcommand_matches("test") {
        println!("Printing testing lists...");
    }
}
