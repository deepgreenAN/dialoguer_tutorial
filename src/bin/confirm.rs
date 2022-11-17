use dialoguer::Confirm;

fn main() -> Result<(), std::io::Error> {
    'main: loop {
        if Confirm::new()
            .with_prompt("プログラムを続けますか?")
            .default(true)
            .interact()?
        {
            println!("プログラムを続けます．");
        } else {
            println!("プログラムを終了します．");
            break 'main;
        }
    }

    Ok(())
}
