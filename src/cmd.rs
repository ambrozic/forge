use ansi_term::Colour;
use yaml_rust::{yaml, Yaml};
use ssh;

const COLORS: &'static [Colour] = &[
    Colour::Yellow,
    Colour::Green,
    Colour::Blue,
    Colour::Purple,
    Colour::Cyan,
];

pub fn run(configuration: &yaml::Yaml, service: &str, pipeline: &str) {
    println!();
    println!("{}", Colour::Yellow.paint("executing"));
    println!("  {} {}", Colour::Green.paint("service"), Colour::Blue.paint(service));
    println!(" {} {}", Colour::Green.paint("pipeline"), Colour::Blue.paint(pipeline));
    ssh::execute(
        &configuration["services"][service]["ssh"]["dsn"].as_str().unwrap(),
        &configuration["services"][service]["ssh"]["key"].as_str().unwrap(),
        &configuration["services"][service]["pipelines"][pipeline].as_vec().unwrap()
    );
}

pub fn list(configuration: &yaml::Yaml) {
    println!();
    fn print_indent(indent: usize) {
        for _ in 0..indent {
            print!("    ");
        }
    }
    fn to_string(v: &yaml::Yaml) -> String {
        match *v {
            Yaml::Boolean(v) => v.to_string(),
            _ => {
                v.as_str().unwrap().to_string()
            }
        }
    }
    fn print_node(name: &str, node: &yaml::Yaml, indent: usize) {
        match node {
            &yaml::Yaml::Array(ref v) => {
                println!("{}:", COLORS[indent - 1].paint(name));
                for x in v {
                    print_indent(indent);
                    println!("- {}", to_string(x));
                }
            },
            &yaml::Yaml::Hash(ref h) => {
                println!("{}:", COLORS[indent - 1].paint(name));
                for (k, v) in h {
                    print_indent(indent);
                    print_node(k.as_str().unwrap(), v, indent + 1);
                }
            },
            _ => {
                println!("{}: {}", COLORS[indent - 1].paint(name), to_string(node));
            },
        }
    }
    print_node("configuration", configuration, 1);
    println!();
}
