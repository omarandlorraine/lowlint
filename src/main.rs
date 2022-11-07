use lang_c::driver::{parse, Config};

mod init;
use crate::init::*;
use lang_c::visit::Visit;

fn main() {
    let config = Config::default();
    let past = parse(&config, "c/uninit.c");
    if let Ok(ref ast) = past {
        let mut check: VarInitCheck = Default::default();
        check.visit_translation_unit(&ast.unit);
    }
    if let Err(err) = past {
        println!("{:?}", err);
    }
}
