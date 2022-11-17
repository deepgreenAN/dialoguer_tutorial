use dialoguer::Input;

fn main() -> Result<(), std::io::Error> {
    let input = Input::<String>::new()
        .with_prompt("名前を入力してください")
        .with_initial_text("name")
        .interact()?;

    println!("ようこそ！{}さん", input);

    Ok(())
}
