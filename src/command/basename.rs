pub(crate) fn basename_command(
    mut multiple: &bool,
    name: &[String],
    suffix: &Option<String>,
    zero: &bool,
) {
    let delimiter = '/';

    for name in name.iter() {
        let split_output = name.rsplit_once(delimiter);
        let mut output = if let Some((_left_string, right_string)) = split_output {
            right_string
        } else {
            name
        };

        if let Some(suffix) = suffix {
            multiple = &true;

            if output.ends_with(&suffix.clone()) {
                output = output.trim_end_matches(&suffix.clone());
            }
        };

        if *zero {
            print!("{output}");
        } else {
            println!("{output}");
        }

        if !*multiple {
            break;
        };
    }
}
