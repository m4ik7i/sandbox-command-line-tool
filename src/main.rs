type UnitResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> UnitResult {
    // let commands = vec![("ls", "")];

    // for (command, args) in commands {
    //     execute(command, args)?
    // }

    execute("ls", "")?;
    execute("rustc", "--version")?;

    Ok(())
}

fn execute(command: &str, args: &str) -> UnitResult {
    let output_binary = std::process::Command::new(command)
        .args(args.split_whitespace().collect::<Vec<_>>())
        .output()?
        .stdout;

    use std::io::Write;
    std::io::stdout().lock().write(&output_binary)?;

    // let output = String::from_utf8(output_binary)?;

    // let mut output_iter = output.split("\n").enumerate();

    // while let Some((index, line)) = output_iter.next() {
    //     if index != 0 {
    //         println!("");
    //     }
    //     print!("{}", line);
    // }

    Ok(())
}
