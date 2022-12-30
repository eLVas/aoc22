use std::env;
use std::fs;
use std::path::PathBuf;
use std::cmp::{Ordering, Reverse};

fn main() {
    let wd = env::current_dir().unwrap();
    println!("Hello, world! From {:?}", wd);

    let file_path = PathBuf::from("day1/resources/input.txt");

    let data_raw = fs::read_to_string(file_path).unwrap();

    let data = data_raw.lines().fold(vec![0], |acc, e| {
        let mut nacc = acc.clone();

        println!("{}", e);

        if e.len() == 0 {
            nacc.push(0)
        } else {
            let val = e.parse::<i32>().unwrap();
            let sum = nacc.pop().unwrap();

            nacc.push(sum + val)
        }

        nacc
    });

    println!("Sum per elf: {:?}", data);

    // Top 1 elf
    let max_cal = data.iter().max().unwrap();
    println!("Max calories (Answer #1): {:?}", max_cal);

    // To also get idx of elf with max calories
    let max_idx_cal = data.iter().enumerate().max_by(|(_, a), (_,b)| a.cmp(b)).unwrap();
    println!("Max calories (elf_idx, sum_calories): {:?}", max_idx_cal);

    // Top 3 elves
    let mut data_sorted = data.clone();
    data_sorted.sort_by_key(|w| Reverse(*w));

    println!("Sorted data {:?}", data_sorted);

    let max_three_cal_sum: &i32 = &data_sorted[0..3].iter().sum();
    println!("Sum calories top three (Answer #2): {:?}", max_three_cal_sum);
}
