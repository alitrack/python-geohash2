pub mod geohash_core;

use geohash_core::GeohashError;
use pyo3::exceptions::{PyUserWarning, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyTuple;

fn map_error(error: GeohashError) -> PyErr {
    match error {
        GeohashError::InvalidCode => {
            PyValueError::new_err("geohash code is [0123456789bcdefghjkmnpqrstuvwxyz]+")
        }
        GeohashError::InvalidArgument => PyValueError::new_err("Invalid argument"),
    }
}

fn warn_if_latitude_adjusted(py: Python<'_>, adjusted: bool) -> PyResult<()> {
    if adjusted {
        PyErr::warn(
            py,
            &py.get_type::<PyUserWarning>(),
            c"latitude 90.0 is outside the geohash latitude range [-90.0, 90.0); encoding the adjacent cell at nextafter(90.0, -inf)",
            1,
        )?;
    }
    Ok(())
}

#[pyfunction]
fn encode(py: Python<'_>, latitude: f64, longitude: f64) -> PyResult<String> {
    let (latitude, adjusted) = geohash_core::normalize_latitude(latitude).map_err(map_error)?;
    warn_if_latitude_adjusted(py, adjusted)?;
    geohash_core::encode_normalized_latitude(latitude, longitude).map_err(map_error)
}

#[pyfunction]
fn decode(hashcode: &str) -> PyResult<(f64, f64, usize, usize)> {
    geohash_core::decode(hashcode).map_err(map_error)
}

#[pyfunction]
fn neighbors(hashcode: &str) -> PyResult<Vec<String>> {
    geohash_core::neighbors(hashcode).map_err(map_error)
}

#[pyfunction]
fn encode_int(py: Python<'_>, latitude: f64, longitude: f64) -> PyResult<(u64, u64)> {
    // Keep the historical Python ABI name; semantically this returns unsigned parts.
    let (latitude, adjusted) = geohash_core::normalize_latitude(latitude).map_err(map_error)?;
    warn_if_latitude_adjusted(py, adjusted)?;
    geohash_core::encode_uint128_parts_normalized_latitude(latitude, longitude).map_err(map_error)
}

#[pyfunction]
#[pyo3(signature = (*args))]
fn decode_int(args: &Bound<'_, PyTuple>) -> PyResult<(f64, f64)> {
    if !matches!(args.len(), 2 | 4 | 8) {
        return Err(PyValueError::new_err(
            "Argument must be 2, 4 or 8 integers.",
        ));
    }

    let mut parts = Vec::with_capacity(args.len());
    for i in 0..args.len() {
        parts.push(args.get_item(i)?.extract::<u64>()?);
    }
    geohash_core::decode_int_parts(&parts).map_err(map_error)
}

#[pymodule]
fn _geohash(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode, m)?)?;
    m.add_function(wrap_pyfunction!(decode, m)?)?;
    m.add_function(wrap_pyfunction!(neighbors, m)?)?;
    m.add_function(wrap_pyfunction!(encode_int, m)?)?;
    m.add_function(wrap_pyfunction!(decode_int, m)?)?;
    m.add("intunit", geohash_core::INT_UNIT)?;
    Ok(())
}
