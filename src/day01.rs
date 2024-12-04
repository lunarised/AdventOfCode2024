pub fn p1(puzzle_input: &str) -> i32 {
    let mut total_distance = 0;
    let list_lines = puzzle_input.lines();
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    for line in list_lines{
        let mut values: std::str::SplitWhitespace<'_> = line.split_whitespace();

        let left_value = values.next().unwrap();
        let right_value = values.next().unwrap();

        left_list.push(left_value.parse::<i32>().unwrap());
        right_list.push(right_value.parse::<i32>().unwrap());    
    }
    left_list.sort();
    right_list.sort();

    for i in 0..left_list.len(){
        total_distance += (left_list[i] - right_list[i]).abs();
    }
    return total_distance;
}

pub fn p2(puzzle_input: &str) -> i32 {
    let mut similarity_score: i32 = 0;
    let list_lines = puzzle_input.lines();
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    for line in list_lines{
        let mut values: std::str::SplitWhitespace<'_> = line.split_whitespace();

        let left_value = values.next().unwrap();
        let right_value = values.next().unwrap();

        left_list.push(left_value.parse::<i32>().unwrap());
        right_list.push(right_value.parse::<i32>().unwrap());    
    }

    for item in left_list {
        
        let mut add_sim = (right_list.iter().filter(|n| **n == item).count() as i32) * item;

        println!("{}", add_sim);
        similarity_score += add_sim;


    }
    return similarity_score;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    #[test]
    fn part1_test() {
        let p1_test_input = utils::read_file("inputs/01ATest.txt".to_string());
        assert_eq!(p1(p1_test_input.as_str()), 11);
    }
    #[test]
    fn part2_test() {
        let p1_test_input = utils::read_file("inputs/01ATest.txt".to_string());
        assert_eq!(p2(p1_test_input.as_str()), 31);
    }
}