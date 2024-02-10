use std::io::{self, BufRead, Write};

pub fn prompt(message: &str) -> io::Result<String> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    stdout.write_all(message.as_bytes())?;
    stdout.flush()?;

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut line = String::new();
    stdin.read_line(&mut line)?;

    line = line.trim().to_string();

    Ok(line)
}
