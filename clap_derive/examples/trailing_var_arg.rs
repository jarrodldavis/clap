//! How to use trailing var args.

use std::ffi::OsString;
use std::path::PathBuf;
use clap::{Clap, AppSettings, ValueHint};

/// Run a program.
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::TrailingVarArg)]
struct Opt {
    /// Record stdout to <file>
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath, value_name = "file")]
    outfile: PathBuf,

    /// Record stderr to <file>
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath, value_name = "file")]
    errfile: PathBuf,

    /// The command to run, with its arguments
    #[clap(required = true, parse(from_os_str), value_hint = ValueHint::CommandWithArguments, name = "cmd-with-args")]
    cmd: Vec<OsString>,
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
