use std::str::FromStr;

pub mod template;

#[macro_export]
macro_rules! printd {
    ($val:expr) => {
        println!("{:?}", $val);
    };
}

pub fn pause() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
}

pub trait LineExtension {
    fn parse_last_word<T: FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug;
}

impl<'a> LineExtension for std::str::Lines<'a> {
    fn parse_last_word<T: FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        self.next()
            .expect("No more lines available")
            .split_whitespace()
            .last()
            .expect("No word found in line")
            .parse::<T>()
            .expect("Failed to parse the last word")
    }
}
