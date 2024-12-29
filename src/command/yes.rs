use core::str;
use std::{
    io::{stdout, BufWriter, Write},
    sync::Arc,
    thread::{self},
    time::{Duration, Instant},
};

use anyhow::Result;

pub(crate) fn yes_command(text: &str, amount: usize, duration: Option<f32>) -> Result<()> {
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
                    writeln!(&mut w, "{}", to_print).unwrap_or_default();
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
                    writeln!(&mut w, "{}", to_print).unwrap_or_default();
                }
            } else {
                let start = Instant::now();
                eprintln!("scheduler starting at {:?}", start);

                let thread_a = thread::spawn(move || {
                    while Instant::now().duration_since(start) < dur {
                        writeln!(&mut w, "{}", to_print).unwrap_or_default();
                    }
                });

                thread_a.join().expect("thread A panicked");
            }
        };
    });

    scheduler.join().expect("scheduler panicked");

    Ok(())
}
