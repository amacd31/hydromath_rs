mod stats;

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

/*
    Kling-Gupta efficiency.

    References:
        * Gupta, Hoshin V., Harald Kling, Koray K. Yilmaz, and Guillermo F.
            Martinez. "Decomposition of the Mean Squared Error and NSE
            Performance Criteria: Implications for Improving Hydrological
            Modelling." Journal of Hydrology 377, no. 1–2 (October 20, 2009):
            80–91. doi:10.1016/j.jhydrol.2009.08.003.
*/
pub fn kge(obs: &[f64], sim: &[f64]) -> f64 {

    let obs_mean: f64 = stats::mean(obs);
    let sim_mean: f64 = stats::mean(sim);
    let obs_std: f64 = stats::standard_dev_mean(obs, obs_mean);
    let sim_std: f64 = stats::standard_dev_mean(sim, sim_mean);

    let beta: f64 = sim_mean / obs_mean;
    let cov_so: f64 = stats::covariance(sim, obs);

    // Default to zero, we set later if sim_std != 0
    let mut alpha: f64 = 0.0;
    let mut r: f64 = 0.0;
    if sim_std != 0.0 {
        alpha = sim_std / obs_std;
        r = cov_so / (sim_std * obs_std);
    }

    return 1.0 - ((r - 1.0).powf(2.0) + (alpha - 1.0).powf(2.0) + (beta - 1.0).powf(2.0)).sqrt();
}
