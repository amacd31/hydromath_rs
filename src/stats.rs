pub(crate) fn mean(data: &[f64]) -> f64 {
    return data.iter().sum::<f64>() / data.len() as f64;
}

#[test]
fn test_mean() {
    let in_data: &[f64; 5] = &[1., 2., 3., 4., 5.];
    let m: f64 = mean(in_data);
    assert_eq!(m, 3.0);
}

pub(crate) fn covariance(data_a: &[f64], data_b: &[f64]) -> f64 {
    let n: f64 = data_a.len() as f64;
    let mean_a: f64 = mean(data_a);
    let mean_b: f64 = mean(data_b);

    let mut variance_a: f64;
    let mut variance_b: f64;
    let mut covariance: f64 = 0f64;
    for (a, b) in data_a.iter().zip(data_b.iter()) {
        variance_a = a - mean_a;
        variance_b = b - mean_b;
        covariance += variance_a * variance_b / n;
    }

    return covariance;
}

pub(crate) fn variance_mean(data: &[f64], mean: f64) -> f64 {
    let mut sum: f64 = 0.0;

    for d in data {
        sum += (d - mean).powf(2.0);
    }

    return sum / data.len() as f64;
}

pub(crate) fn standard_dev_mean(data: &[f64], mean: f64) -> f64 {
    return (variance_mean(data, mean)).sqrt();
}
