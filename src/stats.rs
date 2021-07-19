// Copyright (c) 2021 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

extern crate statrs;
use statrs::distribution::{ContinuousCDF, FisherSnedecor};

pub fn compute_stats(buffer: &std::vec::Vec<f64>) -> (f64, usize, f64, f64, f64, f64, f64) {
    // Compute the mean, sum and standard deviation of the buffer
    let accum = buffer.iter().sum::<f64>() as f64;
    let mean = accum / buffer.len() as f64;
    let squared_differences = (buffer.iter().map(|x| (x - mean) * (x - mean))).sum::<f64>() as f64;
    let std_dev = (squared_differences / (buffer.len() - 1) as f64).sqrt();

    // Compute the median of the array
    let median = if buffer.len() % 2 == 0 {
        let midpoint = buffer.len() / 2;
        (buffer[midpoint + 1] + buffer[midpoint]) / 2.0
    } else {
        buffer[buffer.len() / 2]
    };

    let min = *buffer.first().expect("No minimum value found...");
    let max = *buffer.last().expect("No maximum value found...");

    (mean, buffer.len(), median, std_dev, accum, min, max) // Return value
}

pub fn compute_anova(all_buffers: &Vec<Vec<f64>>) {
    // Compute the sample means and overall means
    let data: Vec<f64> = all_buffers.iter().flat_map(|s| s.iter()).copied().collect();

    // Compute the correction for the mean
    let a_cm = data.iter().sum::<f64>().powf(2.0) / data.len() as f64;
    let a_ssr = data.iter().map(|s| s.powf(2.0)).sum::<f64>() - a_cm;
    let a_sm: Vec<f64> = all_buffers.iter().map(|b| b.iter().sum::<f64>()).collect();
    let a_sst = a_sm
        .iter()
        .zip(all_buffers.iter())
        .map(|(s, b)| s.powf(2.0) / b.len() as f64)
        .sum::<f64>()
        - a_cm;
    let a_sse = a_ssr - a_sst;
    let df_n = all_buffers.len() - 1;
    let df_d = data.len() - all_buffers.len();
    let a_mst = a_sst / df_n as f64;
    let a_mse = a_sse / df_d as f64;
    let a_f = a_mst / a_mse;

    // Compute the critical-value
    let f = FisherSnedecor::new(df_n as f64, df_d as f64).unwrap();
    // Find the signficance value of our F-score with binary search
    let mut min_p = 0.0;
    let mut max_p = 1.0;
    let mut current_p = 0.5;
    while max_p - min_p > 0.0001 {
        if f.inverse_cdf(current_p) > a_f {
            // We can increase the probability
            min_p = current_p;
        } else {
            max_p = current_p;
        }
        current_p = (max_p - min_p) / 2.0;
    }

    println!(
        "One-Way ANOVA F-Score: {} (df1: {}, df2: {}, Significant to p={})",
        a_f,
        all_buffers.len() - 1,
        data.len() - all_buffers.len(),
        current_p
    );
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn simple_test_of_truth() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_compute_stats() {
        let buffer = [1.0, 2.0, 3.0, 4.0, 5.0].to_vec();
        let (mean, count, median, std_dev, accum, min, max) = compute_stats(&buffer);

        assert_eq!(mean, 3.0);
        assert_eq!(count, 5);
        assert_eq!(median, 3.0);
        assert_eq!(std_dev - 1.581138 < 0.00001, true);
        assert_eq!(accum, 15.0);
        assert_eq!(min, 1.0);
        assert_eq!(max, 5.0);
    }
}
