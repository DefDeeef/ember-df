
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = ember::codec::decode_template("H4sI...")?;
    println!("{json}");
    Ok(())
}