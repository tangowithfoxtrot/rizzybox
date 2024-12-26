use std::{thread::sleep, time::Duration};

use anyhow::{bail, Result};

pub(crate) fn sleep_command(sleep_args: &str) -> Result<()> {
    match sleep_args {
        "infinity" => loop {
            sleep(Duration::from_secs(5))
        },
        _ => match sleep_args.parse::<u64>() {
            Ok(s) => {
                sleep(Duration::from_secs(s));
                Ok(())
            }
            Err(_) => bail!("sleep duration must be a number or 'infinity'"),
        },
    }
}
