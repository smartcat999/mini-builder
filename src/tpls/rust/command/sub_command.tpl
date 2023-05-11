use clap::{value_parser, App, Arg, ArgAction, ArgMatches, Command};

pub fn new_sub_command<'help>() -> App<'help> {
    Command::new("hello")
        .about("hello command")
        .arg(
            Arg::new("file")
                .action(ArgAction::Append)
                .value_parser(value_parser!(String))
                .default_values(&["./hello.txt"])
                .short('f')
                .help("文件路径"),
        )
        .override_usage("etool cve -f ./image.txt -p ./tmp -o image.json\n  ")
}

pub fn handler(matches: &ArgMatches) {
    let files: Vec<&String> = matches
        .get_many::<String>("file")
        .unwrap()
        .collect::<Vec<&String>>();
}
