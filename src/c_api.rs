use hydromath::metrics::{nse, mse, rmse, kge};

#[no_mangle]
pub extern "C" fn nse_c(obs_pointer: *const f64, sim_pointer: *const f64, arr_size: usize) -> f64 {

    let obs_arr =
        unsafe { std::slice::from_raw_parts(obs_pointer, arr_size) };
    let sim_arr =
        unsafe { std::slice::from_raw_parts(sim_pointer, arr_size) };

    return nse(obs_arr, sim_arr);
}

#[no_mangle]
pub extern "C" fn mse_c(obs_pointer: *const f64, sim_pointer: *const f64, arr_size: usize) -> f64 {

    let obs_arr =
        unsafe { std::slice::from_raw_parts(obs_pointer, arr_size) };
    let sim_arr =
        unsafe { std::slice::from_raw_parts(sim_pointer, arr_size) };

    return mse(obs_arr, sim_arr);
}

#[no_mangle]
pub extern "C" fn rmse_c(obs_pointer: *const f64, sim_pointer: *const f64, arr_size: usize) -> f64 {

    let obs_arr =
        unsafe { std::slice::from_raw_parts(obs_pointer, arr_size) };
    let sim_arr =
        unsafe { std::slice::from_raw_parts(sim_pointer, arr_size) };

    return rmse(obs_arr, sim_arr);
}

#[no_mangle]
pub extern "C" fn kge_c(obs_pointer: *const f64, sim_pointer: *const f64, arr_size: usize) -> f64 {

    let obs_arr =
        unsafe { std::slice::from_raw_parts(obs_pointer, arr_size) };
    let sim_arr =
        unsafe { std::slice::from_raw_parts(sim_pointer, arr_size) };

    return kge(obs_arr, sim_arr);
}