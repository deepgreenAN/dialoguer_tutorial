use dialoguer::Select;
fn main() -> Result<(), std::io::Error> {
    let choices = vec!["English", "日本語", "中文"];

    let selection = Select::new().items(&choices).interact()?;

    match selection {
        0_usize => println!("Hello!"),
        1_usize => println!("こんにちは！"),
        2_usize => println!("你好!"),
        _ => println!("undefined selection"),
    }
    Ok(())
}
