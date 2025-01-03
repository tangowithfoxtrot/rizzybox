use anyhow::Result;

pub fn dirname_command(name: &[String], zero: bool) -> Result<()> {
    let delimiter = '/';

    for name in name {
        let split_output = name.rsplit_once(delimiter);
        let output = if let Some((left_string, _right_string)) = split_output {
            left_string
        } else {
            "."
        };

        if zero {
            print!("{output}\0");
        } else {
            println!("{output}");
        }
    }
    Ok(())
}
