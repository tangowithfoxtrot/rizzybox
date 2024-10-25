use anyhow::Result;

pub(crate) fn yes_command(text: &str) -> Result<()> {
    // TODO: use a buffer to make this waaaay faster
    loop {
        println!("{text}");
    }
}
