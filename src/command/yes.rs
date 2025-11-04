use core::str;
use std::{
    io::{stdout, BufWriter, Write},
    sync::Arc,
    thread::{self},
    time::{Duration, Instant},
};

pub fn yes_command(text: &str, amount: usize, duration: Option<f32>) {
    let arc = Arc::new(text.to_string());
    let to_print = Arc::clone(&arc);
    let w = BufWriter::new(stdout());

    let scheduler = thread::spawn(move || {
        {
            let mut w = w;
            let to_print = to_print;
            if amount > 0 {
                let mut i = 0;
                while i < amount {
                    writeln!(&mut w, "{to_print}").unwrap_or_default();
                    i += 1;
                }
                return;
            }

            let dur = match Duration::try_from_secs_f32(duration.unwrap_or_default()) {
                Ok(d) => d,
                Err(_) => Duration::ZERO,
            };

            if dur == Duration::ZERO {
                loop {
                    writeln!(&mut w, "{to_print}").unwrap_or_default();
                }
            } else {
                let start = Instant::now();

                let thread_a = thread::spawn(move || {
                    while Instant::now().duration_since(start) < dur {
                        writeln!(&mut w, "{to_print}").unwrap_or_default();
                    }
                });

                thread_a.join().expect("thread A panicked");
            }
        };
    });

    scheduler.join().expect("scheduler panicked");
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use std::time::SystemTime;

    #[allow(unused_imports)]
    use rizzybox::*;

    // Because the `yes` command will run indefinitely by default, tests should always
    // use either the --amount or --duration args so that the command can exit successfully

    #[test]
    fn success() {
        // Arrange
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        // Act
        cmd.arg("yes").arg("--duration").arg("1");

        // Assert
        cmd.assert().success();
    }

    #[test]
    fn amount_outputs_the_specified_amount_of_text() {
        // Arrange
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        // Act
        cmd.arg("yes");
        cmd.arg("--amount");
        cmd.arg("789");

        // Assert
        cmd.assert().success();
        cmd.assert()
            .stdout(predicates::str::contains("y\n".repeat(789)));
    }

    #[test]
    fn duration_runs_for_the_specified_duration() {
        // Arrange
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        // Act
        cmd.arg("yes");
        cmd.arg("--duration");
        cmd.arg("1");

        // Assert
        let start = SystemTime::now();
        cmd.assert().success();
        let elapsed = start.elapsed().unwrap();
        assert!(elapsed.as_secs() >= 1);
        cmd.assert().stdout(predicates::str::contains("y"));
    }
}
