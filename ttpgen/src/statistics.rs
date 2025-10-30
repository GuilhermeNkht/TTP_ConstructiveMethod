// External crates
use plotters::prelude::*;
use log::{info};

pub struct Statistics;

impl Statistics{

    /// Computes the arithmetic mean (average) of a vector of integer values.
    ///
    /// # Arguments
    /// * `data` - A reference to a vector of 128-bit signed integers (`Vec<i128>`)
    ///   representing the values for which the mean will be calculated.
    ///
    /// # Returns
    /// A `f64` value representing the average of all elements in `data`.
    ///
    /// # Panics
    /// This function will **panic** if `data` is empty, because division by zero
    /// would occur. Ensure that the input vector contains at least one value.
    ///
    /// # Example
    /// ```
    /// let values = vec![10_i128, 20, 30, 40];
    /// let avg = mean(&values);
    /// ```
    pub fn mean(data: &Vec<i128>) -> f64 {
        let sum: i128 = data.iter().sum();
        sum as f64 / data.len() as f64
    }

    /// Computes the median value of a vector of integers.
    ///
    /// # Arguments
    /// * `data` - A reference to a vector of `i128` values.
    ///
    /// # Returns
    /// A `f64` representing the median of the input data.
    ///
    /// # Panics
    /// This function will **panic** if `data` is empty, because accessing elements
    /// in an empty slice is invalid. Ensure that the vector contains at least one value.
    ///
    /// # Example
    /// ```
    /// let values = vec![5_i128, 1, 9, 3, 7];
    /// let med = median(&values);
    /// ```
    ///
    pub fn median(data: &Vec<i128>) -> f64 {
        let mut sorted = data.clone();
        sorted.sort();

        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 0 {
            (sorted[mid - 1] as f64 + sorted[mid] as f64) / 2.0
        } else {
            sorted[mid] as f64
        }
    }

    /// Computes the variance of a vector of integer values.
    ///
    /// # Arguments
    /// * `data` - A reference to a vector of `i128` values whose variance will be computed.
    ///
    /// # Returns
    /// A `f64` representing the variance of the data.
    ///
    /// # Panics
    /// This function will **panic** if `data` is empty, since variance is undefined
    /// for an empty dataset. Ensure the input contains at least one value.
    ///
    /// # Example
    /// ```
    /// let values = vec![2_i128, 4, 4, 4, 5, 5, 7, 9];
    /// let var = Statistics::variance(&values);
    /// ```
    ///
    pub fn variance(data: &Vec<i128>) -> f64 {
        let m = Statistics::mean(data);
        data.iter()
            .map(|value| {
                let diff = *value as f64 - m;
                diff * diff
            })
            .sum::<f64>() / data.len() as f64
    }

    /// Computes the standard deviation of a vector of integer values.
    ///
    /// # Arguments
    /// * `data` - A reference to a vector of `i128` values whose standard deviation will be computed.
    ///
    /// # Returns
    /// A `f64` representing the standard deviation.
    ///
    /// # Panics
    /// This function will **panic** if `data` is empty, since standard deviation
    /// cannot be computed without at least one value.
    ///
    /// # Example
    /// ```
    /// let values = vec![2_i128, 4, 4, 4, 5, 5, 7, 9];
    /// let sd = Statistics::std_dev(&values);
    /// ```
    ///
    pub fn std_dev(data: &Vec<i128>) -> f64 {
        Statistics::variance(data).sqrt()
    }

    /// Returns the minimum and maximum values in a vector of integer values.
    ///
    /// # Arguments
    /// * `data` - A reference to a vector of `i128` values.
    ///
    /// # Returns
    /// A tuple `(min, max)`:
    /// - `min` (`i128`): The smallest value in the vector.
    /// - `max` (`i128`): The largest value in the vector.
    ///
    /// # Panics
    /// This function will **panic** if the dataset is empty, because computing a
    /// minimum and maximum requires at least one value.
    ///
    /// # Example
    /// ```
    /// let values = vec![12_i128, 5, 30, 7, 9];
    /// let (min_val, max_val) = Statistics::min_max(&values);
    /// ```
    ///
    pub fn min_max(data: &Vec<i128>) -> (i128, i128) {
        (*data.iter().min().unwrap(), *data.iter().max().unwrap())
    }

    /// Computes the first, second (median), and third quartiles of a vector of integer values.
    ///
    /// # Arguments
    /// * `data` - A reference to a vector of `i128` values.
    ///
    /// # Returns
    /// A tuple `(q1, q2, q3)` of type `(f64, f64, f64)` representing the three quartiles.
    ///
    /// # Panics
    /// This function will **panic** if `data` is empty.
    ///
    /// # Example
    /// ```
    /// let values = vec![7_i128, 15, 36, 39, 40, 41, 42, 43, 47, 49];
    /// let (q1, q2, q3) = Statistics::quartiles(&values);
    /// ```
    pub fn quartiles(data: &Vec<i128>) -> (f64, f64, f64) {
        let mut sorted = data.clone();
        sorted.sort();
        let n = sorted.len();

        let q2 = Statistics::median(&sorted);
        let q1 = Statistics::median(&sorted[..n/2].to_vec());
        let q3 = Statistics::median(&sorted[(n+1)/2..].to_vec());

        (q1, q2, q3)
    }

    /// Plots a histogram of the given distances and saves it as an image file.
    ///
    /// This function divides the range of distances into a fixed number of bins (20),
    /// counts the number of distances falling into each bin, and creates a histogram
    /// chart using the `plotters` crate. The Y-axis is scaled based on the maximum
    /// count plus a margin of 5 (can be changed).
    ///
    /// # Arguments
    /// * `distances` - A reference to a vector of `i128` distances.
    /// * `filename` - A string slice representing the path where the histogram image
    ///   will be saved.
    ///
    /// # Panics
    /// This function will panic if:
    /// - The distances vector is empty.
    /// - Writing the image file fails.
    ///
    /// # Example
    /// ```
    /// let distances = vec![10, 20, 20, 30, 40, 40, 40, 50];
    /// Statistics::plot_histogram(&distances, "output/histogram.png");
    /// ```
    pub fn plot_histogram(distances: &Vec<i128>, filename: &str) {
        let min = *distances.iter().min().unwrap();
        let max = *distances.iter().max().unwrap();

        let root = BitMapBackend::new(filename, (1280, 720))
            .into_drawing_area();
        root.fill(&WHITE).unwrap();

        let bins = 20;
        let step = ((max - min) / bins).max(1);

        let mut counts: Vec<i128> = Vec::new();

        for b in 0..bins {
            let start = min + b * step;
            let end = start + step;

            let count = distances.iter().filter(|&&v| v >= start && v < end).count() as i128;
            counts.push(count);
        }

        let y_max = counts.iter().max().cloned().unwrap_or(0) + 5;

        let mut chart = ChartBuilder::on(&root)
            .caption("Distance Distribution", ("sans-serif", 40))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(min..max, 0..y_max)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        for (b, &count) in counts.iter().enumerate() {
            let start = min + (b as i128) * step;
            let end = start + step;

            chart.draw_series(std::iter::once(Rectangle::new(
                [(start, 0), (end, count)],
                BLUE.mix(0.6).filled(),
            ))).unwrap();
        }
    }

    /// Computes and logs statistical summaries of a vector of distances.
    ///
    /// # Arguments
    /// * `distances` - A reference to a vector of `i128` values representing distances.
    ///
    /// # Example
    /// ```
    /// let distances = vec![10, 20, 30, 40, 50];
    /// Statistics::generate_statistics(&distances);
    /// ```
    pub fn generate_statistics(distances: &Vec<i128>) {

        info!("Mean: {}", Statistics::mean(&distances));
        info!("Median: {}", Statistics::median(&distances));
        info!("Variance: {}", Statistics::variance(&distances));
        info!("Std Dev: {}", Statistics::std_dev(&distances));
        info!("Min-Max: {:?}", Statistics::min_max(&distances));
        info!("Quartiles: {:?}", Statistics::quartiles(&distances));

        Statistics::plot_histogram(&distances, "dist_histogram.png");
    }

}