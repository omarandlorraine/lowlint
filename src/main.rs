use lang_c::driver::{parse, Config};

mod unsafe_fn;
use crate::unsafe_fn::*;
mod init;
use crate::init::*;
use lang_c::visit::Visit;

fn main() {
    let config = Config::default();
    let past = parse(&config, "c/unsafe_functions.c");
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
