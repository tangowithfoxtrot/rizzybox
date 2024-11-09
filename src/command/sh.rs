use anyhow::Result;
use std::env;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::path::Path;
use std::process::Command;

#[cfg(not(target_arch = "riscv64"))] // this doesn't link properly, atm
#[link(name = "c")]
extern "C" {
    fn geteuid() -> u16;
}

pub(crate) fn sh_command() -> Result<()> {
    let mut already_prompted = false;
    loop {
        render_prompt(already_prompted);

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap_or(0);

        let mut cmd_parts = input.split_whitespace();

        let command = cmd_parts.next().unwrap_or_default();
        let mut args = cmd_parts.peekable();
        let args = args.peek();

        match command {
            "" => {
                already_prompted = true;
                continue;
            }
            "cd" => {
                let new_dir = args.map_or(".", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(root) {
                    eprintln!("{}", e);
                }
            }
            "exit" => match args {
                None => return Ok(()),
                Some(_) => std::process::exit(args.unwrap().parse::<i32>().unwrap()),
            },
            command => {
                let status = Command::new(command).args(args).spawn();

                if let Ok(mut status) = status {
                    let _ = status.wait();
                } else {
                    eprintln!("sh: {}: failed to execute command", command)
                }
            }
        }
    }
}

fn render_prompt(already_prompted: bool) {
    if already_prompted {
        print!("\n\r");
    }
    #[cfg(not(target_arch = "riscv64"))]
    if unsafe { geteuid() } == 0 {
        print!("# ");
    } else {
        print!("λ ");
    }
    #[cfg(target_arch = "riscv64")]
    print!("λ ");

    let _ = stdout().flush();
}
