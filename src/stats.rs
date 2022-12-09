pub mod basic_stats;
pub mod curvature;
pub mod save_stats;

// use super::stats::basic_stats;
#[cfg(test)]
mod test {
    use crate::stats::basic_stats;
    #[test]
    fn test_mean() {
        let nums: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(basic_stats::mean(&nums), 3.0);

        let nums_2: Vec<f64> = vec![1.0, 2.0, -3.0, 4.0, -5.0];
        assert_eq!(basic_stats::mean(&nums_2), -0.2);
    }
    #[test]
    fn test_std_dev() {
        let nums: Vec<f64> = vec![1.0, 1.0, 1.0, 1.0, 1.0];
        assert_eq!(basic_stats::std_dev(&nums), 0.0);

        let nums_2: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(basic_stats::std_dev(&nums_2), 2.0_f64.sqrt());
    }
}
