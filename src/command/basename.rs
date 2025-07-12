pub fn basename_command(mut multiple: bool, name: &[String], suffix: Option<&String>, zero: bool) {
    let delimiter = '/';

    for name in name {
        let split_output = name.rsplit_once(delimiter);
        let mut output = if let Some((_, right_string)) = split_output {
            right_string
        } else {
            name
        };

        if let Some(suffix) = suffix {
            multiple = true;

            if output.ends_with(suffix) {
                output = output.trim_end_matches(suffix);
            }
        }

        print!("{output}{}", if zero { "\0" } else { "\n" });

        if !multiple {
            break;
        }
    }
}
