// Copyright (c) 2021 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

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
