use std::env;
use std::io::{self, Write};

const DEFAULT_BUF_SIZE: usize = 64 * 1024;

fn main() -> io::Result<()> {
    let mut expletive = env::args().nth(1).unwrap_or("y".into());
    expletive.push('\n');

    let repeat = DEFAULT_BUF_SIZE / expletive.len();

    let buffer = expletive.repeat(repeat);
    let mut stdout_lock = io::stdout().lock();
    loop {
        stdout_lock.write_all(buffer.as_bytes())?;
    }
}
