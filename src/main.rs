use clap::{crate_name, crate_version, App};
use command_macros::cmd;
use std::ffi::OsStr;
use std::io::{self, Write};
use std::process::{Command, ExitStatus, Output};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let app = App::new(crate_name!()).version(crate_version!());
    let _ = app.get_matches();

    let (status, output) = exec("ls", &[] as &[&str])?;
    if !status.success() {
        write_stderr(&output)?;
    };
    write_stdout(&output)?;

    let (status, output) = exec("rustc", &["--version"])?;
    if !status.success() {
        write_stderr(&output)?;
    };
    write_stdout(&output)?;

    #[rustfmt::skip]
    let output = cmd!(echo ("Hello, World!")).output()?;
    if !output.status.success() {
        write_stderr(&output)?;
    };
    write_stdout(&output)?;

    let output = Command::new("ls").args(&["-l"]).output()?;
    if !output.status.success() {
        write_stderr(&output)?;
    };
    write_stdout(&output)?;

    Ok(())
}

fn exec<C, I, S>(command: C, args: I) -> io::Result<(ExitStatus, Output)>
where
    C: AsRef<OsStr>,
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new(command).args(args).output()?;
    Ok((output.status, output))
}

fn write_stdout(output: &Output) -> io::Result<()> {
    io::stdout().write_all(&output.stdout)?;
    Ok(())
}

fn write_stderr(output: &Output) -> io::Result<()> {
    io::stderr().write_all(&output.stderr)?;
    Ok(())
}
