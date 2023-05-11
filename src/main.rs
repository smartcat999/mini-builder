pub mod command;
pub mod utils;
pub mod libs;
pub mod conf;

extern crate clap;

use clap::App;
use std::env;
use crate::conf::Asset;
use crate::libs::tpl::TeraTplService;


fn main() {
    let current_dir = env::current_dir();
    println!(
        "current_dir: {:?}:",
        current_dir
    );

    let mut tpl = TeraTplService::default();
    for v in Asset::iter() {
        let name = v.parse::<String>().unwrap();
        let content = Asset::get(&name).unwrap();

        match String::from_utf8(content.data.as_ref().to_vec()) {
            Ok(template) => {
                tpl.load_raw_template(&name, &template);
            }
            Err(e) => {
                println!("load template [{:#?}] error: {:#?}", name, e)
            }
        }
    }

    let mut args = App::new("模版生成器")
        .version("v1.0")
        .author("minictl")
        .subcommands(vec![
            command::rs_command::new_sub_command(),
        ])
        .override_usage("minictl <command>\n  ");
    let matches = args.clone().get_matches();
    match matches.subcommand() {
        Some(("rs", matches)) => {
            command::rs_command::handler(matches, tpl.clone());
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
