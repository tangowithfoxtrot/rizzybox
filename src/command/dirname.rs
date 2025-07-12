pub fn dirname_command(name: &[String], zero: bool) {
    let delimiter = '/';

    for name in name {
        let split_output = name.rsplit_once(delimiter);
        let output = if let Some((left_string, _right_string)) = split_output {
            left_string
        } else {
            "."
        };

        print!("{output}{}", if zero { "\0" } else { "\n" });
    }
}
