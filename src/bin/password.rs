use dialoguer::Password;

fn main() -> Result<(), std::io::Error> {
    let password = Password::new()
        .with_prompt("作成するパスワードを入力してください")
        .with_confirmation("再び入力してください", "パスワードが違います")
        .interact()?;

    println!("{}", password);
    Ok(())
}
