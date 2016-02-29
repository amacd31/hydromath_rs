use std::slice;

#[no_mangle]
pub extern fn mse_c (obs_pointer: *const f64,
    sim_pointer: *const f64, arr_size: usize
) -> f64 {

    let obs_arr = unsafe {
            std::slice::from_raw_parts(
                obs_pointer as *const f64,
                arr_size as usize)
    };
    let sim_arr = unsafe {
            std::slice::from_raw_parts(
                sim_pointer as *const f64,
                arr_size as usize)
    };

    return mse( obs_arr, sim_arr);
}

#[no_mangle]
pub extern fn rmse_c (obs_pointer: *const f64,
    sim_pointer: *const f64, arr_size: usize
) -> f64 {

    let obs_arr = unsafe {
            std::slice::from_raw_parts(
                obs_pointer as *const f64,
                arr_size as usize)
    };
    let sim_arr = unsafe {
            std::slice::from_raw_parts(
                sim_pointer as *const f64,
                arr_size as usize)
    };

    return rmse( obs_arr, sim_arr);
}

fn mse(obs: &[f64], sim: &[f64]) -> f64 {
    let mut sum_y: f64 = 0.0;

    for (o, s) in obs.iter().zip(sim.iter()) {
        let delta = s - o;
        sum_y += delta.powf(2.0);
    }
    let len: f64 = obs.len() as f64;
    return sum_y / len;
}

fn rmse(obs: &[f64], sim: &[f64]) -> f64 {
    return mse(obs, sim).sqrt();
}

#[test]
fn test_rmse() {
    let test_obs = &[13.,17.,18.,20.,24.];
    let test_sim = &[12.,15.,20.,22.,24.];

    let result = rmse(test_obs, test_sim);
    assert_eq!(result, (2.6 as f64).sqrt());
}

#[test]
fn test_rmse_perfect() {
    let test_obs = &[1.,2.,3.,4.,5.];
    let test_sim = &[1.,2.,3.,4.,5.];

    let result = rmse(test_obs, test_sim);
    assert_eq!(result, (0 as f64));
}

#[test]
fn test_rmse_bad() {
    let obs: [f64; 5] = [1.,2.,3.,4.,5.];

    let sum: f64 = obs.iter().fold(0f64, std::ops::Add::add);
    let m = sum / (obs.len() as f64);
    let sim = [m, m, m, m, m];
    let result = rmse(
        &[1.,2.,3.,4.,5.],
        &sim
    );

    assert_eq!(result, (2.0 as f64).sqrt());
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

    let sum: f64 = obs.iter().fold(0f64, std::ops::Add::add);
    let m = sum / (obs.len() as f64);
    let sim = [m, m, m, m, m];
    let result = mse(
        &[1.,2.,3.,4.,5.],
        &sim
    );

    assert_eq!(result, 2.0);
}
