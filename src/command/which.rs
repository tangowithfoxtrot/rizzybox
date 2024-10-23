pub(crate) fn which_command(all_occurrences: &bool, bin: &str, silent: &bool) {
    let path = std::env::var("PATH").unwrap();

    let delimiter = ":";
    let paths: Vec<_> = path.split(delimiter).collect();

    let mut was_found: bool = false;
    for path in paths {
        let full_path = format!("{}/{}", path, bin);
        match std::path::Path::new(&full_path).exists() {
            true => {
                was_found = true;
                if !*silent {
                    println!("{}", &full_path);
                }
                if !*all_occurrences {
                    break;
                }
            }
            _ => {
                was_found = match was_found {
                    true => true,
                    false => false,
                };
            }
        };
    }
    if !was_found {
        std::process::exit(1)
    }
}
