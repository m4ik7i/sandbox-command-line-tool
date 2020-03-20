use clap::{crate_name, crate_version, App};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let app = App::new(crate_name!()).version(crate_version!());
    let _ = app.get_matches();

    exec("ls", vec![])?;
    exec("rustc", vec!["--version"])?;

    Ok(())
}

use std::ffi::OsStr;
use std::io::{self, Write};
use std::process::Command;

fn exec<I, S>(command: S, args: I) -> io::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new(command).args(args).output()?;

    io::stdout().lock().write_all(&output.stdout)?;

    if !output.status.success() {
        println!("status: {}", output.status);
        io::stderr().lock().write_all(&output.stderr)?;
    }

    Ok(())
}
