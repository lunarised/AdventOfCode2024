pub fn p1(puzzle_input: &str) -> i32 {
let mut n_safe = 0;
for line in puzzle_input.lines(){
    let is_safe = check_safe(line);
   // println!("{}+++++++==+++++++++++++++++++++++++++++++++++++", is_safe);
    if is_safe == true { n_safe+= 1 }

}


return n_safe;
}


pub fn p2(puzzle_input: &str) -> i32 {
    let mut n_safe = 0;
    for line in puzzle_input.lines(){
        let is_safe = check_safe_dampen(line);
       // println!("{}+++++++==+++++++++++++++++++++++++++++++++++++", is_safe);
        if is_safe == true { n_safe+= 1 }
    
    }
    
    
    return n_safe;
    }
fn check_safe(line: &str) -> bool{
    let mut values = line.split_whitespace();
    let mut last = values.next().unwrap().parse::<i32>().unwrap();
    let mut safe = true;
    let mut direction = 0;

    while(safe){
        let new_value = values.next();
        match (new_value) {
            Some(new_value) => {} ,
            None => return true
        }
        let new_value_int = new_value.unwrap().parse::<i32>().unwrap();

    //    print!("{}, ",new_value_int);

        if new_value_int == last{ return false };
        
        if (new_value_int > last && direction == 0) {direction = 1;} 
        if (new_value_int < last && direction == 0) {direction = -1;} 

        //println!("Hihi");



        if (new_value_int > last && direction == -1) {return false} 
        if (new_value_int < last && direction == 1) {return false}


        //println!("Hihi");

        if ((new_value_int - last).abs() > 3) { return false;} 

        //println!("Hihi")
        last = new_value_int;
    }
return true;
}

fn check_safe_dampen(line: &str) -> bool{
    println!("");
    let mut values = line.split_whitespace();
    let mut last = values.next().unwrap().parse::<i32>().unwrap();
    let mut safe = true;
    let mut direction = 0;
    let mut safety_override = false;
    print!("{} ,", last);
    while(safe){
        let new_value = values.next();
        match (new_value) {
            Some(new_value) => {} ,
            None => return true
        }
        let new_value_int = new_value.unwrap().parse::<i32>().unwrap();
        if (new_value_int > last && direction == 0) {direction = 1;}
        if (new_value_int < last && direction == 0) {direction = -1;}
        let mut problem = false;
        if (new_value_int == last){ problem = true}
        if (new_value_int > last && direction == -1 ) {problem = true; }
        if (new_value_int < last && direction == 1 ) {problem = true; }
        if ((new_value_int - last).abs() > 3 ) {problem = true} 
        print!("{}, {} |", new_value_int, problem);

        // if (!problem){
        // if new_value_int == last && safety_override{ return false };
        // if (new_value_int > last && direction == 0 ) {direction = 1;} 
        // if (new_value_int < last && direction == 0 ) {direction = -1;} 
        // if (new_value_int > last && direction == -1 && safety_override) {return false} 
        // if (new_value_int < last && direction == 1 && safety_override) {return false}
        // if ((new_value_int - last).abs() > 3 && safety_override) { return false} 
        // }
        if (problem && safety_override) {return false}
        if (problem && !safety_override) {safety_override = true}



        last = new_value_int;
    }
return true;
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    #[test]
    fn part1_test() {
        let p1_test_input = utils::read_file("inputs/02t.txt".to_string());
        assert_eq!(p1(p1_test_input.as_str()), 2);
    }
    #[test]
    fn part2_test() {
        let p1_test_input = utils::read_file("inputs/02t.txt".to_string());
        assert_eq!(p2(p1_test_input.as_str()), 4);
    }
}