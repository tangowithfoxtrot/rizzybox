use anyhow::Result;
use rustix::system::uname;

#[derive(Default, Debug, Clone, Copy, clap::ValueEnum)]
/// Enum for ISA format
#[allow(non_camel_case_types)] // otherwise, we'd need serde or something similar
pub enum IsaFormat {
    /// Whatever the system returns
    #[default]
    default,
    /// ISA strings typically used by Docker
    docker,
    /// ISA strings typically used by LLVM
    llvm,
    /// ISA strings typically used by Rust
    rust,
    /// The most basic ISA strings (e.g., x86, arm)
    generic,
}

impl std::fmt::Display for IsaFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

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

    fn format_machine_arch(&self, isa_format: IsaFormat) -> String {
        let native_arch = self.machine.as_str();

        match isa_format {
            IsaFormat::default => native_arch.to_string(),

            IsaFormat::docker => match native_arch {
                "x86_64" => "amd64".to_string(),
                "aarch64" => "arm64".to_string(),
                "armv7l" => "armhf".to_string(),
                "powerpc64le" => "ppc64le".to_string(),
                "i386" | "i686" => "386".to_string(),
                _ => native_arch.to_string(),
            },

            IsaFormat::rust | IsaFormat::llvm => match native_arch {
                "amd64" => "x86_64".to_string(),
                "arm64" => "aarch64".to_string(),
                "armhf" => "armv7".to_string(),
                "ppc64le" => "powerpc64le".to_string(),
                "386" => "i686".to_string(),
                _ => native_arch.to_string(),
            },

            IsaFormat::generic => match native_arch {
                "x86_64" | "amd64" | "i686" | "i386" => "x86".to_string(),
                "aarch64" | "arm64" | "armv7l" => "arm".to_string(),
                "powerpc64le" | "ppc64le" => "ppc".to_string(),
                _ => native_arch.to_string(),
            },
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
        isa_format: IsaFormat,
    ) -> String {
        if all {
            return format!(
                "{} {} {} {} {} {}",
                self.sysname,
                self.nodename,
                self.release,
                self.version,
                match isa_format {
                    IsaFormat::default => self.machine.clone(),
                    _ => self.format_machine_arch(isa_format),
                },
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
                match isa_format {
                    IsaFormat::default => parts.push(self.machine.clone()),
                    _ => parts.push(self.format_machine_arch(isa_format)),
                }
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

#[allow(clippy::too_many_arguments)]
pub fn uname_command(
    all: bool,
    kernel: bool,
    nodename: bool,
    kernel_release: bool,
    kernel_version: bool,
    machine: bool,
    operating_system: bool,
    isa_format: IsaFormat,
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
            operating_system,
            isa_format
        )
    );

    Ok(())
}
