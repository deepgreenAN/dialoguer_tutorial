use dialoguer::Sort;

fn main() -> Result<(), std::io::Error> {
    let list = vec!["command A", "command B", "command C", "command D"];
    let sorted: Vec<usize> = Sort::new()
        .with_prompt("実行する順序を選んでください")
        .items(&list)
        .interact()?;

    for command in sorted.into_iter().map(|i| list[i]) {
        println!("{}を実行しました", command);
    }

    Ok(())
}
