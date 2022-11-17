use std::env;

use lang_c::driver::{parse, Config};

mod unsafe_fn;
use crate::unsafe_fn::*;
mod init;
use crate::init::*;
use lang_c::visit::Visit;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut config = Config::default();

    for arg in &args {
        if &arg[..2] == "-D" {
            config.cpp_options.push(arg.clone());
        }
        if &arg[..2] == "-I" {
            config.cpp_options.push(arg.clone());
        }
    }
    
    for file in args {
        let past = parse(&config, file);
        if let Ok(ref ast) = past {
            let mut unsafe_fn: UnsafeFnCall = Default::default();
            unsafe_fn.visit_translation_unit(&ast.unit);
            for i in unsafe_fn.problems {
                println!("{:?}: {}", i.0, i.1);
            }

            let mut init: VarInitCheck = Default::default();
            init.visit_translation_unit(&ast.unit);
            for i in init.problems {
                println!("{:?}: {}", i.0, i.1);
            }
        }
        if let Err(err) = past {
            println!("{:?}", err);
        }
    }
}
