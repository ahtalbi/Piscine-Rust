pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.len() == 0 || message.contains("stupid") {
        return Err("ERROR: illegal");
    }
    Ok("hello there")
}