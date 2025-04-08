use numpy::PyReadonlyArray1;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use crate::metrics;

macro_rules! create_python_numpy_function {
    ($func_name:ident) => {
        #[pyfunction]
        pub fn $func_name(obs: PyReadonlyArray1<f64>, sim: PyReadonlyArray1<f64>) -> PyResult<f64> {
            let obs = match obs.as_slice() {
                Ok(obs) => obs,
                Err(error) => return Err(PyTypeError::new_err(error)),
            };

            let sim = match sim.as_slice() {
                Ok(sim) => sim,
                Err(error) => return Err(PyTypeError::new_err(error)),
            };

            let skill = metrics::$func_name(obs, sim);

            Ok(skill)
        }
    };
}

create_python_numpy_function!(mse);
create_python_numpy_function!(rmse);
create_python_numpy_function!(nse);
create_python_numpy_function!(kge);

#[pymodule]
pub fn hydromath(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mse, m)?)?;
    m.add_function(wrap_pyfunction!(rmse, m)?)?;
    m.add_function(wrap_pyfunction!(nse, m)?)?;
    m.add_function(wrap_pyfunction!(kge, m)?)?;
    Ok(())
}
