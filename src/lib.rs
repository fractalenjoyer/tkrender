use pyo3::prelude::*;
include!(concat!(env!("OUT_DIR"), "/module.rs"));

#[pyfunction]
fn square(x: isize) -> PyResult<isize> {
    Ok(x * x)
}

#[pyfunction]
fn add2(x: isize, y: isize) -> PyResult<isize> {
    Ok(x + y)
}

#[pyfunction]
fn add3(x: isize, y: isize, z: isize) -> PyResult<isize> {
    Ok(x + y + z)
}

#[pyclass]
struct Bahd {
    x: f64
}

#[pymethods]
impl Bahd {
    #[new]
    fn new(x: f64) -> Self {
        Bahd { x }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Bahd({})", self.x))
    }

    fn square(&self) -> PyResult<f64> {
        Ok(self.x * self.x)
    }
}

#[pyfunction]
fn fib(n: usize) -> PyResult<Vec<usize>> {
    Ok(fibonacci(n))
}

fn fibonacci(n: usize) -> Vec<usize> {
    let mut v = vec![1,1];
    match n {
        1 => vec![1],
        2 => v,
        _ => {
            for i in 2..n {
                v.push(v[i-1] + v[i-2]);
            }
            v
        }
    }
}