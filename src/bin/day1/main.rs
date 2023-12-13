use std::{fs, str::Split};

fn main() {
    let contents = fs::read_to_string("./src/bin/day1/input.txt").expect("Could not read the file");
    let lines = contents.split("\n");
    task_1(lines.clone());
    task_2(lines);
}

fn task_1(lines: Split<'_, &str>) {
    let mut sum: u32 = 0;
    for line in lines {
        let first = line.find(|c: char| c.is_ascii_digit());
        let last = line.rfind(|c: char| c.is_ascii_digit());
        if let (Some(a), Some(b)) = (first, last) {
            let num_a: u32 = (line.as_bytes()[a] - 48).into();
            let num_b: u32 = (line.as_bytes()[b] - 48).into();
            let number: u32 = (num_a * 10) + num_b;
            sum += number;
        }
    }
    println!("Task 1: The answer is {sum}")
}

#[derive(PartialEq, Debug)]
struct WordNum(usize, u32);

fn task_2(lines: Split<'_, &str>) {
    let words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum: u32 = 0;
    for line in lines {
        let first_digit_index = line.find(|c: char| c.is_ascii_digit());
        let last_digit_index = line.rfind(|c: char| c.is_ascii_digit());
        let mut first_word: Option<WordNum> = None;
        let mut last_word: Option<WordNum> = None;
        let mut word_num: u32 = 1;
        for word in &words {
            let fpos = line.find(word);
            if let Some(idx) = fpos {
                if first_word.is_none() {
                    first_word = Some(WordNum(idx, word_num));
                } else if let Some(ref fw) = first_word {
                    if idx < fw.0 {
                        first_word = Some(WordNum(idx, word_num));
                    }
                }
            }
            let lpos = line.rfind(word);
            if let Some(idx) = lpos {
                if last_word.is_none() {
                    last_word = Some(WordNum(idx, word_num));
                } else if let Some(ref lw) = last_word {
                    if idx > lw.0 {
                        last_word = Some(WordNum(idx, word_num));
                    }
                }
            }
            word_num += 1;
        }
        let num_a: u32 = match (first_digit_index, first_word) {
            (Some(fdix), Some(fwix)) if fdix < fwix.0 => (line.as_bytes()[fdix] - 48).into(),
            (Some(fdix), Some(fwix)) if fdix > fwix.0 => fwix.1,
            (Some(fdix), _) => (line.as_bytes()[fdix] - 48).into(),
            (_, Some(fwix)) => fwix.1,
            (_, _) => panic!("No first digit found! {}", line),
        };
        let num_b: u32 = match (last_digit_index, last_word) {
            (Some(ldix), Some(lwix)) if ldix > lwix.0 => (line.as_bytes()[ldix] - 48).into(),
            (Some(ldix), Some(lwix)) if ldix < lwix.0 => lwix.1,
            (Some(ldix), _) => (line.as_bytes()[ldix] - 48).into(),
            (_, Some(lwix)) => lwix.1,
            (_, _) => panic!("No last digit found! {}", line),
        };
        let number: u32 = (num_a * 10) + num_b;
        sum += number;
    }
    println!("Task 2: The answer is {sum}")
}
