use std::{collections::BTreeSet, path::PathBuf};

use anyhow::Result;

pub(crate) fn ls_command(all: &bool, path: &str) -> Result<()> {
    let path_buf = PathBuf::from(path);
    if path_buf.is_file() {
        println!(
            "{}",
            path_buf
                .clone()
                .into_os_string()
                .into_string()
                .expect("pathbuf is file")
        );
        std::process::exit(0);
    }

    let entries = std::fs::read_dir(path_buf)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    let mut file_listings = BTreeSet::new();
    if *all {
        file_listings.insert(".".to_owned());
        file_listings.insert("..".to_owned());
    }
    for entry in entries.iter() {
        if let Some(file_name) = entry
            .clone()
            .into_os_string()
            .into_string()
            .ok()
            .and_then(|s| {
                s.rsplit_once('/')
                    .map(|(_, file_name)| file_name.to_string())
            })
        {
            if *all {
                file_listings.insert(file_name);
            } else if file_name.starts_with('.') {
                continue;
            } else {
                file_listings.insert(file_name);
            }
        }
    }

    for entry in &file_listings {
        println!("{}", entry);
    }
    Ok(())
}
