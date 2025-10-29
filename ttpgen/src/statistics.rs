use plotters::prelude::*;
pub struct Statistics;

impl Statistics{

    pub fn mean(data: &Vec<i128>) -> f64 {
        let sum: i128 = data.iter().sum();
        sum as f64 / data.len() as f64
    }

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

    pub fn variance(data: &Vec<i128>) -> f64 {
        let m = Statistics::mean(data);
        data.iter()
            .map(|value| {
                let diff = *value as f64 - m;
                diff * diff
            })
            .sum::<f64>() / data.len() as f64
    }

    pub fn std_dev(data: &Vec<i128>) -> f64 {
        Statistics::variance(data).sqrt()
    }

    pub fn min_max(data: &Vec<i128>) -> (i128, i128) {
        (*data.iter().min().unwrap(), *data.iter().max().unwrap())
    }

    pub fn quartiles(data: &Vec<i128>) -> (f64, f64, f64) {
        let mut sorted = data.clone();
        sorted.sort();
        let n = sorted.len();

        let q2 = Statistics::median(&sorted);
        let q1 = Statistics::median(&sorted[..n/2].to_vec());
        let q3 = Statistics::median(&sorted[(n+1)/2..].to_vec());

        (q1, q2, q3)
    }

    pub fn plot_histogram(distances: &Vec<i128>, filename: &str) {
        let min = *distances.iter().min().unwrap();
        let max = *distances.iter().max().unwrap();

        let root = BitMapBackend::new(filename, (1280, 720))
            .into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption("Distance Distribution", ("sans-serif", 40))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(min..max, 0..distances.len() as i128)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        let bins = 20;
        let step = ((max - min) / bins).max(1);

        for b in 0..bins {
            let start = min + b * step;
            let end = start + step;

            let count = distances.iter().filter(|&&v| v >= start && v < end).count() as i128;

            chart.draw_series(std::iter::once(Rectangle::new(
                [(start, 0), (end, count)],
                BLUE.mix(0.6).filled(),
            ))).unwrap();
        }

        println!("Histograma salvo em {}", filename);
    }

    pub fn generate_statistics(distances: &Vec<i128>) {

        println!("Mean: {}", Statistics::mean(&distances));
        println!("Median: {}", Statistics::median(&distances));
        println!("Variance: {}", Statistics::variance(&distances));
        println!("Std Dev: {}", Statistics::std_dev(&distances));
        println!("Min-Max: {:?}", Statistics::min_max(&distances));
        println!("Quartiles: {:?}", Statistics::quartiles(&distances));

        Statistics::plot_histogram(&distances, "dist_histogram.png");
    }

}