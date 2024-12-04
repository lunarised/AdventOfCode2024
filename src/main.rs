mod day01;
mod utils;

fn main() {
    let text_input_1 = utils::read_file("inputs/01.txt".to_string());
    println!("{}", day01::p1(text_input_1.as_str()));
    println!("{}", day01::p2(text_input_1.as_str()));
}
