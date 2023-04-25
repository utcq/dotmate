use crate::Errors;
use crate::pre;
use crate::engine;

use std::fs;


struct Parser;
#[allow(unused_parens)]
impl Parser {
    fn install(args: Vec<String>) {
        if (args.len()<3) {
            match fs::metadata(".mate") {
                Ok(metadata) => {
                    if metadata.is_file() {
                        engine::finstall();
                    } else {
                        Errors::wfile_install();
                    }
                },
                Err(_) => {
                    Errors::wfile_install();
                }
            }
        }
        else {
            engine::install(&args[2]);
        }
    }
}


#[allow(unused_parens)]
pub fn parse(args: Vec<String>) {
    match (args[1].as_str()) {
        "install" =>       {  Parser::install(args)                  }
        "help"    =>       {  println!("{}", pre::help_gen())        }
        &_        =>       {  Errors::unknown_cmd(args[1].as_str())  }
    }
}
