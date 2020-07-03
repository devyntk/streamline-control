//! This job builds rust source from static files and templates,
//! which can then be `include!`d in `main.rs`.
use ructe::{Ructe, RucteError};

fn main() -> Result<(), RucteError> {
    let mut ructe = Ructe::from_env()?;
    let mut statics = ructe.statics()?;
    statics.add_files("static")?;
    ructe.compile_templates("templates")
}