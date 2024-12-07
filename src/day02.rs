use core::error;

pub fn p1(puzzle_input: &str) -> i32 {


let mut n_safe = 0;
for line in puzzle_input.lines(){
    let mut values = line.split_whitespace();
    let values_vector: Vec<&str> = values.clone().collect();
    let series: Vec<i32> = values_vector.clone().iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let is_safe = check_safe(series);

    if is_safe == true { n_safe+= 1 }

}


return n_safe;
}


pub fn p2(puzzle_input: &str) -> i32 {
    let mut n_safe = 0;
    for line in puzzle_input.lines(){
        let values = line.split_whitespace();
        let values_vector: Vec<&str> = values.collect();
        let series: Vec<i32> = values_vector.clone().iter().map(|x| x.parse::<i32>().unwrap()).collect();
        
    
       let is_safe = check_safe_dampen(series);
       if !is_safe {
        println!("{:?} {}", line, is_safe);
       }
       if is_safe == true { n_safe+= 1 }
    
    }
    
    
    return n_safe;
    }
fn check_safe(line: Vec<i32>) -> bool{
    let mut values = line.iter();
    let mut last = values.next().unwrap();
    let mut safe = true;
    let mut direction = 0;

    while(safe){
        let new_value = values.next();
        match (new_value) {
            Some(new_value) => {} ,
            None => return true
        }
        let new_value_int = new_value.unwrap();

        if new_value_int == last{ return false };
        
        if (new_value_int > last && direction == 0) {direction = 1;} 
        if (new_value_int < last && direction == 0) {direction = -1;} 


        if (new_value_int > last && direction == -1) {return false} 
        if (new_value_int < last && direction == 1) {return false}

        if ((new_value_int - last).abs() > 3) { return false;} 


        last = new_value_int;
    }
return true;
}

fn check_safe_dampen(line: Vec<i32>) -> bool{
    let mut series: Vec<i32> = line.clone();

    let mut direction = 0;
    let mut error_index: i32 = -1;
    for i in 1..series.len(){
      if i == 0 { continue; }

      if error_index != -1 {
        if direction == -1 {
            if series[i] > series[i-2]{
                series.remove(i - 2);
                return check_safe(series);
            }
            else{
                series.remove(i - 1);
                return check_safe(series);
            }
        }
        if direction == 1 {
            if series[i] < series[i-2]{
                
                series.remove(i - 2);
                return check_safe(series);
            }
            else{
                series.remove(i - 1);
                return check_safe(series);
            }


        }
      }
      
      
      if series[i] > series[i-1]{
        if direction == 0 { direction = 1}
        if direction == -1 { error_index = i as i32;
        let mut left_remove = series.clone();
        let mut right_remove = series.clone();
        left_remove.remove(i-1);
        right_remove.remove(i);
        return (check_safe(left_remove) || check_safe(right_remove));
        }

      } 

      if series[i] < series[i-1]{
        if direction == 0 { direction = -1}
        if direction == 1 {error_index = i as i32;
            let mut left_remove = series.clone();
            let mut right_remove = series.clone();
            left_remove.remove(i-1);
            right_remove.remove(i);
            return (check_safe(left_remove) || check_safe(right_remove));
        }
      }

      for j in 0..i{
          if series[i] == series[j]{
              let mut series_b = series.clone();
              series.remove(i);
              series_b.remove(j);
              println!("DDD");
              return check_safe(series) || check_safe(series_b);

      }
    }
      
    


    //Within 3
    if (series[i] - series[i-1]).abs() > 3{
        let mut series_b = series.clone();

        series.remove(i);
        series_b.remove(i-1);
        return check_safe(series) || check_safe(series_b);
    }

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

    #[test]
    fn debug(){
        let inp = vec![46 ,43 ,44, 43, 42];
        println!("{}", check_safe_dampen(inp.clone()));
        assert!(check_safe_dampen(inp.clone()));
    }
}