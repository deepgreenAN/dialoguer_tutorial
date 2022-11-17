use dialoguer::History;
use dialoguer::Input;
use std::collections::VecDeque;

struct MyHistory(VecDeque<String>);

impl MyHistory {
    fn new() -> Self {
        Self(VecDeque::new())
    }
}

impl<T: ToString> History<T> for MyHistory {
    fn read(&self, pos: usize) -> Option<String> {
        self.0.get(pos).cloned()
    }
    fn write(&mut self, val: &T) {
        self.0.push_front(val.to_string());
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut history = MyHistory::new();
    'main: loop {
        let input = Input::<String>::new()
            .with_prompt("あなたの名前を入力してください")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.chars().count() <= 10 {
                    Ok(())
                } else {
                    Err("名前は10文字以内にしてください．")
                }
            })
            .history_with(&mut history)
            .interact_text()?;

        if &input == "exit" {
            break 'main;
        }
        println!("ようこそ！{}", input);
    }

    Ok(())
}
