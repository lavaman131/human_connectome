pub fn mean(nums: &Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;
    for n in nums.iter() {
        sum += n;
    }
    sum / nums.len() as f64
}

pub fn std_dev(nums: &Vec<f64>) -> f64 {
    let mean: f64 = mean(&nums);
    let mut sum: f64 = 0.0;
    for n in nums.iter() {
        sum += (n - mean).powf(2.0);
    }
    (sum / nums.len() as f64).sqrt()
}
