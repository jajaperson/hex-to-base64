use io::prelude::*;
use std::io;

fn hex_to_base64(hex: &str) -> Result<String, hex::FromHexError> {
    Ok(base64::encode(hex::decode(hex)?))
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let hex: String = buffer
        .chars()
        .take_while(|ch| ch.is_ascii_hexdigit())
        .collect();

    let b64: String = hex_to_base64(&hex).expect("Should be valid hex.");

    println!("{}", b64);

    Ok(())
}
