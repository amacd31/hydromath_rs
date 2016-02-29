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

pub fn mse(obs: &[f64], sim: &[f64]) -> f64 {
    let mut sum_y: f64 = 0.0;

    for (o, s) in obs.iter().zip(sim.iter()) {
        let delta = s - o;
        sum_y += delta.powf(2.0);
    }
    let len: f64 = obs.len() as f64;
    return sum_y / len;
}

pub fn rmse(obs: &[f64], sim: &[f64]) -> f64 {
    return mse(obs, sim).sqrt();
}
