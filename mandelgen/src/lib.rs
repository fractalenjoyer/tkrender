use pyo3::prelude::*;
use numpy::ndarray::{Array3, Dim};
use numpy::{IntoPyArray, PyArray};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

fn calc(a: f64, b: f64) -> i32 {
    let (mut zr, mut zi, mut zr2, mut zi2) = (0.0, 0.0, 0.0, 0.0);
    for i in 1..10000 {
        zi = (zi + zi) * zr + b;
        zr = zr2 - zi2 + a;
        zr2 = zr * zr;
        zi2 = zi * zi;
        if zr2 + zi2 > 1e10 {
            return i as i32;
        }
    }
    return 0 as i32;
}

#[pyfunction]
fn get_mandel<'py>(py:Python<'py>, dim: usize) -> PyResult<&PyArray<u8, Dim<[usize; 3]>>> {
    let mut image = Array3::<u8>::zeros([dim, dim, 3]);
    let (mut a, mut b): (f64, f64);
    for x in 1..dim {
        for y in 1..dim {
            a = 3. * (x as f64) / (dim as f64) - 2.;
            b = 3. * (y as f64) / (dim as f64) - 1.5;
            image[[x, y, 0]] = calc(a, b) as u8;
        }
    }
    Ok(image.into_pyarray(py))
}
/// A Python module implemented in Rust.
#[pymodule]
fn mandelgen(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(get_mandel, m)?)?;
    Ok(())
}