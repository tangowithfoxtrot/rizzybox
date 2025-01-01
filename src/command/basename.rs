use anyhow::Result;

pub(crate) fn basename_command(
    mut multiple: &bool,
    name: &[String],
    suffix: &Option<String>,
    zero: &bool,
) -> Result<()> {
    let delimiter = '/';

    for name in name.iter() {
        let split_output = name.rsplit_once(delimiter);
        let mut output = if let Some((_, right_string)) = split_output {
            right_string
        } else {
            name
        };

        if let Some(suffix) = suffix {
            multiple = &true;

            if output.ends_with(suffix) {
                output = output.trim_end_matches(suffix);
            }
        };

        if *zero {
            print!("{output}\0");
        } else {
            println!("{output}");
        }

        if !*multiple {
            break;
        };
    }
    Ok(())
}
