fn mse(obs: &[f64], sim: &[f64]) -> f64 {
    let mut sum_y: f64 = 0.0;

    for (o, s) in obs.iter().zip(sim.iter()) {
        let delta = s - o;
        sum_y += delta.powf(2.0);
    }
    let len: f64 = obs.len() as f64;
    return sum_y / len;
}

#[test]
fn test_mse() {
    let test_obs = &[13.,17.,18.,20.,24.];
    let test_sim = &[12.,15.,20.,22.,24.];

    let result = mse(test_obs, test_sim);
    assert_eq!(result, 2.6);
}

#[test]
fn test_mse_perfect() {
    let result = mse(&[1.,2.,3.,4.,5.],
                  &[1.,2.,3.,4.,5.]);

    assert_eq!(result, 0.0);
}

#[test]
fn test_mse_bad() {
    let obs: [f64; 5] = [1.,2.,3.,4.,5.];

    // Hard coded because I haven't worked out how to sum Vec in rust successfully
    let sum: f64 = 15.0;
    let m = sum / (obs.len() as f64);
    let sim = [m, m, m, m, m];
    let result = mse(
        &[1.,2.,3.,4.,5.],
        &sim
    );

    assert_eq!(result, 2.0);
}
