use dialoguer::MultiSelect;

fn main() -> Result<(), std::io::Error> {
    let choices = vec!["プログラミング", "Rust", "Python", "数学", "歴史"];
    let defaults = vec![true, false, false, false, false];

    let selections: Vec<usize> = MultiSelect::new()
        .with_prompt("タグを選んでください")
        .items(&choices)
        .defaults(&defaults)
        .interact()?;

    let tags = selections.iter().map(|i| choices[*i]).collect::<Vec<_>>();
    println!("タグは{:?}です．", tags);

    Ok(())
}
