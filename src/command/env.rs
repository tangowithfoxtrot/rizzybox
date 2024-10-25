use anyhow::Result;
use bat::PrettyPrinter;
use std::collections::HashMap;

pub(crate) fn env_command() -> Result<()> {
    let mut environment: HashMap<String, String> = HashMap::new();
    let mut kv_pairs = String::new();

    for (key, value) in std::env::vars() {
        let kv_pair = format!("{}={}\n", key, value);
        kv_pairs.push_str(&kv_pair);
        environment.insert(key, value);
    }

    PrettyPrinter::new()
        .input_from_bytes(kv_pairs.as_bytes())
        .language("env")
        .print()
        .unwrap();
    Ok(())
}
