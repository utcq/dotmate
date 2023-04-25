use std::env;

mod err;
use err::Errors;

mod pre;
mod packages;
mod engine;

mod argsmgr;

#[allow(unused_parens)]
fn main() {
    let args: Vec<String> = env::args().collect();

    if (args.len()<2) {
        Errors::missing_cmd();
    }

    argsmgr::parse(args);
}
