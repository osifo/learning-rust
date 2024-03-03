use core::num;

fn sum(nums: &[i64]) -> i64 {
  let mut total = 0;
  // let numbers: &Vec<i64> = &nums;

  for num in nums.iter() {
    total += num
  }

  println!("total ========== {}", total);
  return total;
}

fn product(nums: &Vec<i64>) -> (i64, &Vec<i64>) {
  let mut product_val  = 1;
 
  let numbers: &Vec<i64> = &nums;

  for number in numbers.iter() {
    product_val  *= number;
  } 
  return (product_val, nums);
}

fn average(nums: &[i64]) {
   let length = nums.len();
   let avg = sum(nums) / length as i64;

   println!("average ======== {}", avg);
}

fn first_three<'a, 'b>(numbers1: &'a Vec<i64>, numbers2: &'b Vec<i64>) -> (&'a [i64], &'b [i64]) {
  let slice1: &[i64] = &numbers1[0..3];
  let slice2: &[i64] = &numbers2[0..3];

  return (slice1, slice2);
}

fn main() {
  let numbers: Vec<i64> = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26];
  let other_numbers: Vec<i64> = vec![101, 302, 240, 4023, 901];


  sum(&numbers);
  let (product_val, _) = product(&numbers);

  println!("product val ==== {}", product_val);

  average(&numbers);

  let (a, b) = first_three(&numbers, &other_numbers);
  println!("the first three numbers are");

  for num in a.iter() {
    println!("{}", num)
  }

  for num in b.iter() {
    println!("{}", num)
  }
}