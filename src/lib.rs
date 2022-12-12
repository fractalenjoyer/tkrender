use pyo3::prelude::*;
include!(concat!(env!("OUT_DIR"), "/module.rs"));
use ndarray::arr2;
use rayon::prelude::*;

#[pyclass]
struct Shape {
    points: Vec<Vec<f64>>,
}

#[pymethods]
impl Shape {
    #[new]
    fn new(points: Vec<Vec<f64>>) -> Self {
        Self { points }
    }
    fn rotate(&self, angle_x: f64, angle_y: f64, angle_z: f64) -> PyResult<Vec<Vec<f64>>> {
        let cos_x = angle_x.cos();
        let sin_x = angle_x.sin();
        let cos_y = angle_y.cos();
        let sin_y = angle_y.sin();
        let cos_z = angle_z.cos();
        let sin_z = angle_z.sin();
        let rotation_matrix = arr2(&[
            [
                cos_y * cos_z,
                -cos_x * sin_z + sin_x * sin_y * cos_z,
                sin_x * sin_z + cos_x * sin_y * cos_z,
            ],
            [
                cos_y * sin_z,
                cos_x * cos_z + sin_x * sin_y * sin_z,
                -sin_x * cos_z + cos_x * sin_y * sin_z,
            ],
            [-sin_y, sin_x * cos_y, cos_x * cos_y],
        ]);
        Ok(self
            .points
            .par_iter()
            .map(|point_vec| {
                let point = arr2(&[[point_vec[0]], [point_vec[1]], [point_vec[2]]]);
                rotation_matrix.dot(&point).into_raw_vec()
            })
            .collect::<Vec<Vec<f64>>>())
    }
    fn rotate_in_place(&mut self, angle_x: f64, angle_y: f64, angle_z: f64) -> PyResult<()> {
        self.points = self.rotate(angle_x, angle_y, angle_z)?;
        Ok(())
    }
    fn get_points(&self) -> PyResult<Vec<Vec<f64>>> {
        Ok(self.points.clone())
    }
    fn get_view(&self, focal: f64, origin: Vec<f64>) -> PyResult<Vec<Vec<f64>>> {
        let camera = arr2(&[[focal, 0.0, 0.0], [0.0, focal, 0.0], [0.0, 0.0, 1.0]]);
        let origin = arr2(&[[origin[0]], [origin[1]], [origin[2]]]);
        Ok(self
            .points
            .par_iter()
            .map(|point_vec| {
                let point = arr2(&[[point_vec[0]], [point_vec[1]], [point_vec[2]]]) - &origin;
                let view = camera.dot(&point).into_raw_vec();
                vec![view[0], view[1]]
            })
            .collect::<Vec<Vec<f64>>>())
    }
}

#[pyclass]
struct Engine {
    shapes: Vec<Py<Shape>>,
}

#[pymethods]
impl Engine {
    #[new]
    fn new(shapes: Vec<Py<Shape>>) -> Self {
        Self { shapes }
    }
    fn get_view(&self, py: Python<'_>, focal: f64, origin: Vec<f64>) -> PyResult<Vec<Vec<f64>>> {
        Ok(self
            .shapes
            .iter()
            .map(|shape| shape.borrow(py).get_view(focal, origin.clone()).unwrap())
            .reduce(|a, b| [a, b].concat())
            .unwrap())
    }
}
