use anyhow::Result;

pub(crate) fn expand_command(mut file: &str, tabs: &Option<Vec<String>>) -> Result<()> {
    if file == "-" {
        file = "/dev/stdin";
    }

    let content = std::fs::read_to_string(file);

    let mut total_spaces = String::new();
    for tab in tabs.clone().unwrap().iter() {
        let repeated = tab.parse::<usize>();
        if let Ok(repeated) = repeated {
            total_spaces.push_str(&" ".repeat(repeated));
        } else {
            break; // TODO: handle '+' and '/' cases
        }
    }

    match content {
        Ok(_) => {
            let modified_content = content.unwrap().replace('\t', &total_spaces);
            println!("{modified_content}");
        }
        Err(_) => std::process::exit(1),
    }
    Ok(())
}
