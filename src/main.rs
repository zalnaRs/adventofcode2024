use std::fs;
use std::fs::File;
use std::io::{BufRead, Read};

fn main() {
    let input = fs::read_to_string("input.real").expect("input.real error");

    let mut left_list = vec![];
    let mut right_list = vec![];
    let mut list = vec![];

    for i in input.split_whitespace() {
        let num: i32 = i.parse().expect("Failed to parse integer");
        list.push(num);
        if list.len() == 2 {
            left_list.push(list[0]);
            right_list.push(list[1]);
            list.clear();
        }
    }

    left_list.sort();
    right_list.sort();

    let mut score = 0;

    for left_item in left_list.iter() {
        let mut ntimes = 0;
        for right_item in right_list.iter() {
            if left_item==right_item {
                ntimes+=1;
            }
        }

        score += ntimes*left_item
    }

    /*let mut results = vec![];
    for i in 0..left_list.len() {
        results.push((left_list[i] - right_list[i]).abs());
    }

    let distance: i32 = results.iter().sum();
  println!("{distance}");*/
    println!("{score}")
}
