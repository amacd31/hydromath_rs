use numpy::PyReadonlyArray1;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use crate::metrics;

#[pyfunction]
pub fn mse(obs: PyReadonlyArray1<f64>, sim: PyReadonlyArray1<f64>) -> PyResult<f64> {
    let obs = match obs.as_slice() {
        Ok(obs) => obs,
        Err(error) => return Err(PyTypeError::new_err(error)),
    };

    let sim = match sim.as_slice() {
        Ok(sim) => sim,
        Err(error) => return Err(PyTypeError::new_err(error)),
    };

    let skill = metrics::mse(obs, sim);

    Ok(skill)
}

#[pyfunction]
pub fn rmse(obs: PyReadonlyArray1<f64>, sim: PyReadonlyArray1<f64>) -> PyResult<f64> {
    let obs = match obs.as_slice() {
        Ok(obs) => obs,
        Err(error) => return Err(PyTypeError::new_err(error)),
    };

    let sim = match sim.as_slice() {
        Ok(sim) => sim,
        Err(error) => return Err(PyTypeError::new_err(error)),
    };

    let skill = metrics::rmse(obs, sim);

    Ok(skill)
}

#[pyfunction]
pub fn nse(obs: PyReadonlyArray1<f64>, sim: PyReadonlyArray1<f64>) -> PyResult<f64> {
    let obs = match obs.as_slice() {
        Ok(obs) => obs,
        Err(error) => return Err(PyTypeError::new_err(error)),
    };

    let sim = match sim.as_slice() {
        Ok(sim) => sim,
        Err(error) => return Err(PyTypeError::new_err(error)),
    };

    let skill = metrics::nse(obs, sim);

    Ok(skill)
}

#[pyfunction]
pub fn kge(obs: PyReadonlyArray1<f64>, sim: PyReadonlyArray1<f64>) -> PyResult<f64> {
    let obs = match obs.as_slice() {
        Ok(obs) => obs,
        Err(error) => return Err(PyTypeError::new_err(error)),
    };

    let sim = match sim.as_slice() {
        Ok(sim) => sim,
        Err(error) => return Err(PyTypeError::new_err(error)),
    };

    let skill = metrics::kge(obs, sim);

    Ok(skill)
}

#[pymodule]
pub fn hydromath(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mse, m)?)?;
    m.add_function(wrap_pyfunction!(rmse, m)?)?;
    m.add_function(wrap_pyfunction!(nse, m)?)?;
    m.add_function(wrap_pyfunction!(kge, m)?)?;
    Ok(())
}
