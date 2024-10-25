use anyhow::Result;
use libc::uname;
use std::{collections::HashSet, env::consts::OS, ffi::CStr};

#[derive(Debug)]
struct UtsName {
    sysname: String,
    nodename: String,
    release: String,
    version: String,
    machine: String,
}

impl UtsName {
    fn new() -> Result<UtsName, &'static str> {
        let mut utsname = unsafe { std::mem::zeroed() };

        if unsafe { uname(&mut utsname) } != 0 {
            return Err("Failed to get uname information");
        }

        Ok(UtsName {
            sysname: unsafe {
                CStr::from_ptr(utsname.sysname.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            },
            nodename: unsafe {
                CStr::from_ptr(utsname.nodename.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            },
            release: unsafe {
                CStr::from_ptr(utsname.release.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            },
            version: unsafe {
                CStr::from_ptr(utsname.version.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            },
            machine: unsafe {
                CStr::from_ptr(utsname.machine.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            },
        })
    }
}

pub(crate) fn arch_command() -> Result<()> {
    let _ = uname_command(&false, &false, &false, &false, &false, &true, &false);
    Ok(())
}

// rizzybox uname:
// user@hostname ~> uname -a
// Linux hostname 6.1.0-26-cloud-amd64 #1 SMP PREEMPT_DYNAMIC Debian 6.1.112-1 (2024-09-30) x86_64 Linux
// GNU uname:
// user@hostname ~>  /usr/bin/uname -a
// Linux hostname 6.1.0-26-cloud-amd64 #1 SMP PREEMPT_DYNAMIC Debian 6.1.112-1 (2024-09-30) x86_64 GNU/Linux
pub(crate) fn uname_command(
    all: &bool,
    kernel: &bool,
    nodename: &bool,
    kernel_release: &bool,
    kernel_version: &bool,
    machine: &bool,
    operating_system: &bool,
) -> Result<()> {
    match UtsName::new() {
        Ok(utsname) => {
            if *all {
                println!(
                    "{} {} {} {} {} {}",
                    utsname.sysname,
                    utsname.nodename,
                    utsname.release,
                    utsname.version,
                    utsname.machine,
                    OS
                );
                return Ok(());
            } else {
                let mut to_print = HashSet::new();
                // FIXME: this is stinky. do something better to workaround the fact that *kernel is a default arg
                if *kernel
                    && !*nodename
                    && !*kernel_release
                    && !*kernel_version
                    && !*machine
                    && !*operating_system
                {
                    to_print.insert(utsname.sysname);
                }
                if *nodename {
                    to_print.insert(utsname.nodename);
                }
                if *kernel_release {
                    to_print.insert(
                        utsname
                            .release
                            .split_ascii_whitespace()
                            .last()
                            .unwrap()
                            .to_string(),
                    );
                }
                if *kernel_version {
                    to_print.insert(utsname.version);
                }
                if *machine {
                    to_print.insert(utsname.machine);
                }
                if *operating_system {
                    // TODO: figure out how to get this at runtime
                    to_print.insert(OS.to_string());
                }
                println!(
                    "{}",
                    to_print
                        .into_iter()
                        .collect::<Vec<String>>()
                        .join(" ")
                        .trim_end()
                );
            }
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
    Ok(())
}
