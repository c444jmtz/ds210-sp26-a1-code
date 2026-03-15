use std::collections::HashMap;

fn sum_even(v: Vec<i32>) -> i32 {
  // Your code will go here
    let mut sum= 0; 
    for i in v {
        if  (i % 2) == 0 {
            sum += i    
        }
        
    }
  return sum;

}

fn main() {
  let v = vec![2, 4, 5, 3, 7, 8, 10, 11];
  let sum = sum_even(v);
  println!("The sum of the even numbers is {sum}");
}

#[test]
fn test1() {
  let v = vec![1, 2, 5, 6, 7];
  let sum = sum_even(v);
  assert_eq!(sum, 8);
}

#[test]
fn test2() {
  let v = vec![13, 5, 17, 3];
  let sum = sum_even(v);
  assert_eq!(sum, 0);
}