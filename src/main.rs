fn multiply_1(num: Option<usize>) -> usize {
  if num.is_some() {
    return num.unwrap() * 5;
  }
  return 0;
}

fn multiply_2(num: Option<usize>) -> usize {
  if let Some(x) = num {
    return x * 5;
  }
  return 0;
}

fn multiply_3(num: Option<usize>) -> usize {
  return num.unwrap_or(0) * 5;
}

fn multiply_4(num: Option<usize>) -> Option<usize> {
  return num.map(|x| x*5);
}

fn multiply_5(num: Option<usize>) -> Option<usize> {
  let num = num?;
  return Some(num * 5);
}

fn multiply_6(num: Option<usize>) -> Option<usize> {
  return Some(num? * 5);
}

fn run_multiply() {
  let num = 5;
  // all the multiplies above give the same result
  let resp = multiply_1(Some(num));
  println!("{}", resp);
}

//  ----------------------- SMALL PROGRAM ------------------------------- 
// let's do a moment of practice

fn practice_1(nums: Vec<usize>, idx: usize) -> usize {
  if nums.get(idx).is_some() {
    return nums.get(idx).unwrap() * 5;
  }
  return idx * 5;
}

fn practice_2(nums: Vec<usize>, idx: usize) -> usize {
  if let Some(x) = nums.get(idx) {
    return x * 5;
  }
  return idx * 5;
}

fn practice_3(nums: Vec<usize>, idx: usize) -> usize {
  return nums.get(idx).unwrap_or(&idx) * 5;
}

fn main() {
  let numbers_1 = vec![1,2,3,4,5];
  let resp_1 = practice_1(numbers_1, 1);
  println!("{}", resp_1); // resp_1 -> 10
}