#[no_mangle]
pub extern "C" fn nse_c(obs_pointer: *const f64, sim_pointer: *const f64, arr_size: usize) -> f64 {

    let obs_arr =
        unsafe { std::slice::from_raw_parts(obs_pointer as *const f64, arr_size as usize) };
    let sim_arr =
        unsafe { std::slice::from_raw_parts(sim_pointer as *const f64, arr_size as usize) };

    return nse(obs_arr, sim_arr);
}

#[no_mangle]
pub extern "C" fn mse_c(obs_pointer: *const f64, sim_pointer: *const f64, arr_size: usize) -> f64 {

    let obs_arr =
        unsafe { std::slice::from_raw_parts(obs_pointer as *const f64, arr_size as usize) };
    let sim_arr =
        unsafe { std::slice::from_raw_parts(sim_pointer as *const f64, arr_size as usize) };

    return mse(obs_arr, sim_arr);
}

#[no_mangle]
pub extern "C" fn rmse_c(obs_pointer: *const f64, sim_pointer: *const f64, arr_size: usize) -> f64 {

    let obs_arr =
        unsafe { std::slice::from_raw_parts(obs_pointer as *const f64, arr_size as usize) };
    let sim_arr =
        unsafe { std::slice::from_raw_parts(sim_pointer as *const f64, arr_size as usize) };

    return rmse(obs_arr, sim_arr);
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

/*
    Nash-Sutcliffe efficiency.

    References:
        * Nash, Jea, and J. V. Sutcliffe. 1970. "River Flow Forecasting through
             Conceptual Models Part I-A Discussion of Principles." Journal of
             Hydrology 10 (3): 282-90.
*/
pub fn nse(obs: &[f64], sim: &[f64]) -> f64 {

    let m: f64 = obs.iter().fold(0f64, std::ops::Add::add) / (obs.len() as f64);

    //int t;
    let mut e1: f64 = 0.0;
    let mut e2: f64 = 0.0;

    for (o, s) in obs.iter().zip(sim.iter()) {
        let delta = o - s;
        e1 += delta.powf(2.0);

        let bias = o - m;
        e2 += bias.powf(2.0);
    }

    if e1 == 0.0f64 {
        return 1.0f64;
    } else {
        return 1.0f64 - e1 / e2;
    }
}
