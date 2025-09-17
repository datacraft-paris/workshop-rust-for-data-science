mod EASY;
mod HARD;
mod CORRECTION;
mod INTERMEDIATE;
mod lib_v0;

use crate::CORRECTION::{lib_v1, lib_v2, lib_v3, lib_v4};
use crate::EASY::{lib_v1EASY, lib_v2EASY, lib_v3EASY, lib_v4EASY};
use crate::HARD::{lib_v1HARD, lib_v2HARD, lib_v3HARD, lib_v4HARD};
use crate::INTERMEDIATE::{lib_v1INTERMEDIATE, lib_v2INTERMEDIATE, lib_v3INTERMEDIATE, lib_v4INTERMEDIATE};


use pyo3::prelude::*;

use ndarray::Array1;
use ndarray_linalg::Scalar;
use numpy::{PyArray1, PyReadonlyArray1, ToPyArray};

#[pymodule]
pub fn poly_match_rs(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    let v0 = PyModule::new_bound(py, "v0")?;
    lib_v0::poly_match_rs(py, &v0)?;
    m.add_submodule(&v0)?;

    let v1 = PyModule::new_bound(py, "v1")?;
    lib_v1::poly_match_rs(py, &v1)?;
    m.add_submodule(&v1)?;

    let v1EASY = PyModule::new_bound(py, "v1EASY")?;
    lib_v1EASY::poly_match_rs(py, &v1EASY)?;
    m.add_submodule(&v1EASY)?;

    let v1HARD = PyModule::new_bound(py, "v1HARD")?;
    lib_v1HARD::poly_match_rs(py, &v1HARD)?;
    m.add_submodule(&v1HARD)?;

    let v1INTERMEDIATE = PyModule::new_bound(py, "v1INTERMEDIATE")?;
    lib_v1INTERMEDIATE::poly_match_rs(py, &v1INTERMEDIATE)?;
    m.add_submodule(&v1INTERMEDIATE)?;

    let v2 = PyModule::new_bound(py, "v2")?;
    lib_v2::poly_match_rs(py, &v2)?;
    m.add_submodule(&v2)?;

    let v2EASY = PyModule::new_bound(py, "v2EASY")?;
    lib_v2EASY::poly_match_rs(py, &v2EASY)?;
    m.add_submodule(&v2EASY)?;

    let v2HARD = PyModule::new_bound(py, "v2HARD")?;
    lib_v2HARD::poly_match_rs(py, &v2HARD)?;
    m.add_submodule(&v2HARD)?;

    let v2INTERMEDIATE = PyModule::new_bound(py, "v2INTERMEDIATE")?;
    lib_v2INTERMEDIATE::poly_match_rs(py, &v2INTERMEDIATE)?;
    m.add_submodule(&v2INTERMEDIATE)?;

    let v3 = PyModule::new_bound(py, "v3")?;
    lib_v3::poly_match_rs(py, &v3)?;
    m.add_submodule(&v3)?;

    let v3EASY = PyModule::new_bound(py, "v3EASY")?;
    lib_v3EASY::poly_match_rs(py, &v3EASY)?;
    m.add_submodule(&v3EASY)?;

    let v3HARD = PyModule::new_bound(py, "v3HARD")?;
    lib_v3HARD::poly_match_rs(py, &v3HARD)?;
    m.add_submodule(&v3HARD)?;

    let v3INTERMEDIATE = PyModule::new_bound(py, "v3INTERMEDIATE")?;
    lib_v3INTERMEDIATE::poly_match_rs(py, &v3INTERMEDIATE)?;
    m.add_submodule(&v3INTERMEDIATE)?;

    let v4 = PyModule::new_bound(py, "v4")?;
    lib_v4::poly_match_rs(py, &v4)?;
    m.add_submodule(&v4)?;

    let v4EASY = PyModule::new_bound(py, "v4EASY")?;
    lib_v4EASY::poly_match_rs(py, &v4EASY)?;
    m.add_submodule(&v4EASY)?;

    let v4HARD = PyModule::new_bound(py, "v4HARD")?;
    lib_v4HARD::poly_match_rs(py, &v4HARD)?;
    m.add_submodule(&v4HARD)?;

    let v4INTERMEDIATE = PyModule::new_bound(py, "v4INTERMEDIATE")?;
    lib_v4INTERMEDIATE::poly_match_rs(py, &v4INTERMEDIATE)?;
    m.add_submodule(&v4INTERMEDIATE)?;




    Ok(())
}
