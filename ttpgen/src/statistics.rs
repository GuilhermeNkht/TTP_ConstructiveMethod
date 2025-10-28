use std::cmp::Ordering;
pub struct Statistics;

impl Statistics{

    pub fn generate_statistics(distances: &Vec<i128>) {

        let n = distances.len();
        let sum: i128 = distances.iter().sum();
        let mean = sum as f64 / n as f64;

        let mut sorted = distances.clone();
        sorted.sort_unstable();

        let median = if n % 2 == 0 {
            (sorted[n / 2 - 1] + sorted[n / 2]) as f64 / 2.0
        } else {
            sorted[n / 2] as f64
        };

        let min = sorted.first().unwrap();
        let max = sorted.last().unwrap();

        let variance = distances.iter()
            .map(|x| {
                let diff = *x as f64 - mean;
                diff * diff
            })
            .sum::<f64>() / n as f64;
        let std_dev = variance.sqrt();

        let q1 = sorted[n / 4] as f64;
        let q3 = sorted[(3 * n) / 4] as f64;
        let iqr = q3 - q1;

        println!("Count: {}", n);
        println!("Sum: {}", sum);
        println!("Mean: {:.2}", mean);
        println!("Median: {:.2}", median);
        println!("Min: {}", min);
        println!("Max: {}", max);
        println!("Variance: {:.2}", variance);
        println!("Std. Deviation: {:.2}", std_dev);
        println!("Q1: {:.2}", q1);
        println!("Q3: {:.2}", q3);
        println!("IQR: {:.2}", iqr);
    }


}