use anyhow::Result;

pub(crate) fn false_command() -> Result<()> {
    std::process::exit(1);
}
