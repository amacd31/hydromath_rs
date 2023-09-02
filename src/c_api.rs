use crate::metrics;

#[no_mangle]
pub extern "C" fn nse(obs_pointer: *const f64, sim_pointer: *const f64, arr_size: usize) -> f64 {

    let obs_arr =
        unsafe { std::slice::from_raw_parts(obs_pointer, arr_size) };
    let sim_arr =
        unsafe { std::slice::from_raw_parts(sim_pointer, arr_size) };

    return metrics::nse(obs_arr, sim_arr);
}

#[no_mangle]
pub extern "C" fn mse(obs_pointer: *const f64, sim_pointer: *const f64, arr_size: usize) -> f64 {

    let obs_arr =
        unsafe { std::slice::from_raw_parts(obs_pointer, arr_size) };
    let sim_arr =
        unsafe { std::slice::from_raw_parts(sim_pointer, arr_size) };

    return metrics::mse(obs_arr, sim_arr);
}

#[no_mangle]
pub extern "C" fn rmse(obs_pointer: *const f64, sim_pointer: *const f64, arr_size: usize) -> f64 {

    let obs_arr =
        unsafe { std::slice::from_raw_parts(obs_pointer, arr_size) };
    let sim_arr =
        unsafe { std::slice::from_raw_parts(sim_pointer, arr_size) };

    return metrics::rmse(obs_arr, sim_arr);
}

#[no_mangle]
pub extern "C" fn kge(obs_pointer: *const f64, sim_pointer: *const f64, arr_size: usize) -> f64 {

    let obs_arr =
        unsafe { std::slice::from_raw_parts(obs_pointer, arr_size) };
    let sim_arr =
        unsafe { std::slice::from_raw_parts(sim_pointer, arr_size) };

    return metrics::kge(obs_arr, sim_arr);
}