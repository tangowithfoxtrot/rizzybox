use std::{
    fs::File,
    io::{BufWriter, Write},
};

use anyhow::Result;

pub(crate) fn yes_command(text: &str) -> Result<()> {
    let buf = File::create("/dev/stdout")?;
    let mut w = BufWriter::new(buf);

    loop {
        writeln!(&mut w, "{}", text)?;
    }
}
