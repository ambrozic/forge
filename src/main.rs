extern crate ansi_term;
extern crate clap;
extern crate ssh2;
extern crate yaml_rust;

use std::fs::File;
use std::io::prelude::*;
use clap::{App, Arg, SubCommand};
use yaml_rust::yaml;

mod cmd;
mod ssh;


fn main() {
    let args = App::new("forge")
        .arg(
            Arg::with_name("file")
                .help("Sets a custom configuration file")
                .short("f")
                .long("file")
                .value_name("file")
                .default_value("forge.yml")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("verbose")
                .help("Sets the level of verbosity")
                .short("v")
                .short("verbose")
                .multiple(true)
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Run service pipelines")
                .arg(
                    Arg::with_name("service")
                        .help("Service name")
                        .short("s")
                        .long("service")
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::with_name("pipeline")
                        .help("Pipeline name")
                        .short("p")
                        .long("pipeline")
                        .index(2)
                        .required(true)
                )
        )
        .subcommand(SubCommand::with_name("list").about("List available services and pipelines"))
        .get_matches();


    let file = args.value_of("file").unwrap();
    let mut s = String::new();
    File::open(&file).unwrap().read_to_string(&mut s).unwrap();
    let configuration = &yaml::YamlLoader::load_from_str(&s).unwrap()[0];

    if let Some(ref args) = args.subcommand_matches("run") {
        cmd::run(
            &configuration,
            &args.value_of("service").unwrap(),
            &args.value_of("pipeline").unwrap(),
        );
    }
    if let Some(_) = args.subcommand_matches("list") {
        cmd::list(&configuration);
    }
}
