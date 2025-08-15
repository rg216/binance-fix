use std::{env, io};

use quickfix_msg_gen::*;

const SPEC_FILENAME: &str = "src/spot-fix-oe.xml";
const BEGIN_STRING: &str = "FIX.4.4";

fn main() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").expect("Missing env var OUT_DIR");
    generate(SPEC_FILENAME, format!("{out_dir}/code.rs"), BEGIN_STRING)?;
    Ok(())
}
