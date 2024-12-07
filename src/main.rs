mod day01;
mod day02;
mod utils;

fn main() {
    let text_input_1 = utils::read_file("inputs/01.txt".to_string());
    println!("{}", day01::p1(text_input_1.as_str()));
    println!("{}", day01::p2(text_input_1.as_str()));
    let text_input_2 = utils::read_file("inputs/02.txt".to_string());
    println!("{}", day02::p1(text_input_2.as_str()));
    println!("{}", day02::p2(text_input_2.as_str()));
}
