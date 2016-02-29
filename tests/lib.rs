extern crate hydromath;

#[test]
fn test_rmse() {
    let test_obs = &[13.,17.,18.,20.,24.];
    let test_sim = &[12.,15.,20.,22.,24.];

    let result = hydromath::rmse(test_obs, test_sim);
    assert_eq!(result, (2.6 as f64).sqrt());
}

#[test]
fn test_rmse_perfect() {
    let test_obs = &[1.,2.,3.,4.,5.];
    let test_sim = &[1.,2.,3.,4.,5.];

    let result = hydromath::rmse(test_obs, test_sim);
    assert_eq!(result, (0 as f64));
}

#[test]
fn test_rmse_bad() {
    let obs: [f64; 5] = [1.,2.,3.,4.,5.];

    let sum: f64 = obs.iter().fold(0f64, std::ops::Add::add);
    let m = sum / (obs.len() as f64);
    let sim = [m, m, m, m, m];
    let result = hydromath::rmse(
        &[1.,2.,3.,4.,5.],
        &sim
    );

    assert_eq!(result, (2.0 as f64).sqrt());
}

#[test]
fn test_mse() {
    let test_obs = &[13.,17.,18.,20.,24.];
    let test_sim = &[12.,15.,20.,22.,24.];

    let result = hydromath::mse(test_obs, test_sim);
    assert_eq!(result, 2.6);
}

#[test]
fn test_mse_perfect() {
    let result = hydromath::mse(&[1.,2.,3.,4.,5.],
                  &[1.,2.,3.,4.,5.]);

    assert_eq!(result, 0.0);
}

#[test]
fn test_mse_bad() {
    let obs: [f64; 5] = [1.,2.,3.,4.,5.];

    let sum: f64 = obs.iter().fold(0f64, std::ops::Add::add);
    let m = sum / (obs.len() as f64);
    let sim = [m, m, m, m, m];
    let result = hydromath::mse(
        &[1.,2.,3.,4.,5.],
        &sim
    );

    assert_eq!(result, 2.0);
}
