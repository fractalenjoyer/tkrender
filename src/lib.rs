use pyo3::prelude::*;
include!(concat!(env!("OUT_DIR"), "/module.rs"));
use ndarray::arr2;

#[pyclass]
struct Engine {
    polygon: Vec<Vec<f64>>,
}

#[pymethods]
impl Engine {
    #[new]
    fn new(polygon: Vec<Vec<f64>>) -> Self {
        Self { polygon }
    }

    fn get_polygon(&self) -> PyResult<Vec<Vec<f64>>> {
        Ok(self.polygon.clone())
    }

    fn get_view(&self) -> PyResult<Vec<Vec<f64>>> {
        Ok(self
            .polygon
            .clone()
            .iter()
            .filter(|point| point[2].is_normal())
            .map(|point| vec![point[0] / point[2], point[1] / point[2]])
            .collect::<Vec<Vec<f64>>>())
    }

    fn rotate_x(&mut self, angle_rad: f64) -> PyResult<()> {
        let cos = angle_rad.cos();
        let sin = angle_rad.sin();
        let rotation_matrix = arr2(&[[1., 0., 0.], [0., cos, -sin], [0., sin, cos]]);
        self.polygon = self
            .polygon
            .iter()
            .map(|point_vec| {
                let point = arr2(&[[point_vec[0]], [point_vec[1]], [point_vec[2]]]);
                rotation_matrix.dot(&point).into_raw_vec()
            })
            .collect::<Vec<Vec<f64>>>();
        Ok(())
    }
    fn rotate_y(&mut self, angle_rad: f64) -> PyResult<()> {
        let cos = angle_rad.cos();
        let sin = angle_rad.sin();
        let rotation_matrix = arr2(&[[cos, 0., sin], [0., 1., 0.], [-sin, 0., cos]]);
        self.polygon = self
            .polygon
            .iter()
            .map(|point_vec| {
                let point = arr2(&[[point_vec[0]], [point_vec[1]], [point_vec[2]]]);
                rotation_matrix.dot(&point).into_raw_vec()
            })
            .collect::<Vec<Vec<f64>>>();
        Ok(())
    }
    fn rotate_z(&mut self, angle_rad: f64) -> PyResult<()> {
        let cos = angle_rad.cos();
        let sin = angle_rad.sin();
        let rotation_matrix = arr2(&[[cos, -sin, 0.], [sin, cos, 0.], [0., 0., 1.]]);
        self.polygon = self
            .polygon
            .iter()
            .map(|point_vec| {
                let point = arr2(&[[point_vec[0]], [point_vec[1]], [point_vec[2]]]);
                rotation_matrix.dot(&point).into_raw_vec()
            })
            .collect::<Vec<Vec<f64>>>();
        Ok(())
    }
    fn rotate(&mut self, x_rad: f64, y_rad: f64, z_rad: f64) -> PyResult<()> {
        if x_rad != 0. {
            self.rotate_x(x_rad)?;
        }
        if y_rad != 0. {
            self.rotate_y(y_rad)?;
        }
        if z_rad != 0. {
            self.rotate_z(z_rad)?;
        }
        Ok(())
    }
}
