pub(crate) fn yes_command(text: &str) {
    // TODO: use a buffer to make this waaaay faster
    loop {
        println!("{text}");
    }
}
