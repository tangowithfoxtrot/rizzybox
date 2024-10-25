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

pub(crate) fn arch_command() {
    uname_command(&false, &false, &false, &false, &false, &true, &false);
}

pub(crate) fn uname_command(
    all: &bool,
    kernel: &bool,
    nodename: &bool,
    kernel_release: &bool,
    kernel_version: &bool,
    machine: &bool,
    operating_system: &bool,
) {
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
                std::process::exit(0);
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
                std::process::exit(0);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
