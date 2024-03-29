use std::{fs, path::Path, process};
use std::collections::HashMap;
use std::rc::Rc;
use clap::{value_parser, App, Arg, ArgAction, ArgMatches, Command};
use crate::libs::basic::{Constructor};
use crate::libs::constants::{Element, ELE_DIR, ELE_RUST_CODE, ELE_RUST_MOD, ELE_RUST_TOML, ELE_RUST_MAKEFILE};
use crate::libs::constructor::EleConstructorV1;
use crate::libs::rs::basic::{EleRustCodeRender, EleRustMakefileRender, EleRustModRender, EleRustTomlRender};
use crate::libs::tpl::TeraTplService;

// const TPL_PREFIX: &str = "rust/command/";

pub fn new_sub_command<'help>() -> App<'help> {
    Command::new("rs")
        .about("rust命令行工具项目脚手架")
        .arg(
            Arg::new("name")
                .default_values(&["excel-tool"])
                .long("name")
                .help("生成的命令行工具项目名"),
        )
        .arg(
            Arg::new("version")
                .default_value("v1.0")
                .long("version")
                .short('v')
                .help("版本信息"),
        )
        .arg(
            Arg::new("author")
                .long("author")
                .default_value("smartcat")
                .help("作者"),
        )
        .override_usage("minictl rs --name excel-tool -v v1.0 --author smartcat\n  ")
}

pub fn handler(matches: &ArgMatches, tpl: TeraTplService) {
    let name = matches.get_one::<String>("name").unwrap();
    let version = matches.get_one::<String>("version").unwrap();
    let author = matches.get_one::<String>("author").unwrap();

    if name == "/" {
        process::exit(1);
    }
    let mut path: String;

    path = name.to_string();
    if !name.ends_with('/') {
        path += "/";
    }

    if !Path::exists(Path::new(&path)) {
        fs::create_dir_all(&path).unwrap();
    };

    let mut constructor: EleConstructorV1 = EleConstructorV1::default();

    let mut data: HashMap<String, String> = HashMap::new();
    data.insert(String::from("name"), name.to_string());
    data.insert(String::from("version"), version.to_string());
    data.insert(String::from("author"), author.to_string());

    let elements = r#"
    [
        {
        "path": "src/main.rs",
        "element_type": 1,
        "template_name": "rust/command/command.tpl"
        },
        {
        "path": "src/command",
        "element_type": 3,
        "template_name": "rust/command/mod.tpl"
        },
        {
        "path": "src/command/hello.rs",
        "element_type": 1,
        "template_name": "rust/command/sub_command.tpl"
        },
        {
        "path": "Cargo.toml",
        "element_type": 2,
        "template_name": "rust/command/Cargo.tpl"
        },
        {
        "path": "Makefile",
        "element_type": 4,
        "template_name": "rust/Makefile.tpl"
        }
    ]"#;

    let elements: Vec<Element> = serde_json::from_str(elements).unwrap();

    let tpl = Rc::new(tpl);
    for elem in elements.iter() {
        if elem.element_type == ELE_RUST_CODE {
            constructor.insert(Box::new(EleRustCodeRender::new(
                path.clone() + &elem.path,
                data.clone(),
                elem.template_name.clone(),
                tpl.clone(),
            )));
        } else if elem.element_type == ELE_RUST_MOD {
            constructor.insert(
                Box::new(EleRustModRender::new(
                    path.to_string() + &elem.path,
                    data.clone(),
                    elem.template_name.clone(),
                    tpl.clone(),
                ))
            )
        } else if elem.element_type == ELE_RUST_TOML {
            constructor.insert(
                Box::new(EleRustTomlRender::new(
                    path.to_string() + &elem.path,
                    data.clone(),
                    elem.template_name.clone(),
                    tpl.clone(),
                ))
            )
        } else if elem.element_type == ELE_RUST_MAKEFILE {
            constructor.insert(
                Box::new(EleRustMakefileRender::new(
                    path.to_string() + &elem.path,
                    data.clone(),
                    elem.template_name.clone(),
                    tpl.clone(),
                ))
            )
        }
    }

    let ret = constructor.build();
    match ret {
        Ok(()) => {}
        Err(err) => {
            println!("constructor build error: {:#?}", err);
        }
    }
}
