#[no_mangle]
pub extern fn mse(obs: Vec<f32>, sim: Vec<f32>) -> f32 {
    let mut sum_y: f32 = 0.0;

    for (o, s) in obs.iter().zip(sim.iter()) {
        let delta = s - o;
        sum_y += delta.powf(2.0);
    }
    let len: f32 = obs.len() as f32;
    return sum_y / len;
}

#[test]
fn test_mse() {
    let test_obs = vec![13.,17.,18.,20.,24.];
    let test_sim = vec![12.,15.,20.,22.,24.];

    let result = mse(test_obs, test_sim);
    assert_eq!(result, 2.6);
}

#[test]
fn test_mse_perfect() {
    let result = mse(vec![1.,2.,3.,4.,5.],
                  vec![1.,2.,3.,4.,5.]);

    assert_eq!(result, 0.0);
}

#[test]
fn test_mse_bad() {
    let obs: Vec<f32> = vec![1.,2.,3.,4.,5.];

    // Hard coded because I haven't worked out how to sum Vec in rust successfully
    let sum: f32 = 15.0;
    let m = sum / (obs.len() as f32);
    let sim = vec![m, m, m, m, m];
    let result = mse(
        vec![1.,2.,3.,4.,5.],
        sim
    );

    assert_eq!(result, 2.0);
}
