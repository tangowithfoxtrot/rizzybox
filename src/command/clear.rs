use anyhow::Result;

pub fn clear_command() -> Result<()> {
    println!("\x1b[2J\x1b[H");
    Ok(())
}
