use anyhow::Result;
use rustix::system::uname;

#[derive(Debug)]
struct UtsName {
    sysname: String,
    nodename: String,
    release: String,
    version: String,
    machine: String,
}

impl UtsName {
    /// Create a new UtsName struct from the system's uname information
    fn new() -> Result<Self> {
        let uname = uname();
        Ok(Self {
            sysname: uname.sysname().to_string_lossy().into_owned(),
            nodename: uname.nodename().to_string_lossy().into_owned(),
            release: uname.release().to_string_lossy().into_owned(),
            version: uname.version().to_string_lossy().into_owned(),
            machine: uname.machine().to_string_lossy().into_owned(),
        })
    }

    /// Get the operating system name based on runtime information
    fn get_os_string(&self) -> String {
        match self.sysname.as_str() {
            "Linux" => {
                // Check for Android
                if std::path::Path::new("/system/bin/adb").exists()
                    || std::path::Path::new("/system/build.prop").exists()
                {
                    "Android".to_string()
                } else {
                    // Check for common GNU/Linux indicators
                    if std::path::Path::new("/usr/bin/gnu-gcc").exists()
                        || std::path::Path::new("/etc/debian_version").exists()
                        || std::path::Path::new("/etc/redhat-release").exists()
                    {
                        "GNU/Linux".to_string()
                    } else {
                        "Linux".to_string()
                    }
                }
            }
            "Darwin" => {
                if self.machine.starts_with("iPhone") || self.machine.starts_with("iPad") {
                    "iOS".to_string()
                } else {
                    "Darwin".to_string()
                }
            }
            "FreeBSD" => "FreeBSD".to_string(),
            "DragonFly" => "DragonFly".to_string(),
            "OpenBSD" => "OpenBSD".to_string(),
            "SunOS" => "Solaris".to_string(),
            "Windows" => "💩".to_string(),
            other => other.to_string(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    /// Format output according to requested flags
    fn format_output(
        &self,
        all: bool,
        kernel: bool,
        nodename: bool,
        kernel_release: bool,
        kernel_version: bool,
        machine: bool,
        operating_system: bool,
    ) -> String {
        if all {
            return format!(
                "{} {} {} {} {} {}",
                self.sysname,
                self.nodename,
                self.release,
                self.version,
                self.machine,
                self.get_os_string()
            );
        }

        let mut parts = Vec::new();

        let no_args_passed = !kernel
            && !nodename
            && !kernel_release
            && !kernel_version
            && !machine
            && !operating_system;

        // Default to kernel name if no flags specified
        if no_args_passed {
            parts.push(self.sysname.clone());
        } else {
            // Add parts according to flags
            if kernel {
                parts.push(self.sysname.clone());
            }
            if nodename {
                parts.push(self.nodename.clone());
            }
            if kernel_release {
                parts.push(self.release.clone());
            }
            if kernel_version {
                parts.push(self.version.clone());
            }
            if machine {
                parts.push(self.machine.clone());
            }
            if operating_system {
                parts.push(self.get_os_string());
            }
        }

        parts.join(" ")
    }
}

pub fn arch_command() -> Result<()> {
    let utsname = UtsName::new()?;
    println!("{}", utsname.machine);
    Ok(())
}

pub fn uname_command(
    all: bool,
    kernel: bool,
    nodename: bool,
    kernel_release: bool,
    kernel_version: bool,
    machine: bool,
    operating_system: bool,
) -> Result<()> {
    let utsname = UtsName::new()?;
    println!(
        "{}",
        utsname.format_output(
            all,
            kernel,
            nodename,
            kernel_release,
            kernel_version,
            machine,
            operating_system
        )
    );

    Ok(())
}
