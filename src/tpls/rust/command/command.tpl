pub mod command;

extern crate clap;
use clap::App;

fn main() {
    let mut args = App::new("{{ name }}")
        .version("{{ version }}")
        .author("{{ author }}")
        .subcommands(vec![
            command::hello::new_sub_command(),
        ])
        .override_usage("{{ name }} <command>\n  ");
    let matches = args.clone().get_matches();
    match matches.subcommand() {
        Some(("hello", matches)) => {
            command::hello::handler(matches);
        }
        _ => {
            match args.print_help() {
                Ok(ret) => ret,
                Err(err) => {
                    println!("{:#?}", err);
                }
            };
        }
    };
}
