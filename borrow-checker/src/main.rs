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

fn main() {
  let numbers = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26];

  sum(&numbers);
  let (product_val, _) = product(&numbers);

  println!("product val ==== {}", product_val);

  average(&numbers)
}