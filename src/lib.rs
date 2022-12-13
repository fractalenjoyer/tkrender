use pyo3::prelude::*;
include!(concat!(env!("OUT_DIR"), "/module.rs"));
use ndarray::arr2;
use rayon::prelude::*;

#[pyclass]
struct Shape {
    points: Vec<Vec<f64>>,
    faces: Vec<Vec<usize>>,
}

#[pymethods]
impl Shape {
    #[new]
    fn new(path: String) -> Self {
        let contents = std::fs::read_to_string(path).unwrap();
        let mut points = Vec::new();
        let mut faces = Vec::new();
        for line in contents.lines() {
            let mut line_iter = line.split_whitespace();
            match line_iter.next() {
                Some("v") => {
                    let items = line_iter.collect::<Vec<&str>>();
                    let x = items[0].parse::<f64>().unwrap();
                    let y = items[1].parse::<f64>().unwrap();
                    let z = items[2].parse::<f64>().unwrap();
                    points.push(vec![x, y, z]);
                }
                Some("f") => {
                    // TODO: Support more than 3 vertices per face
                    let items = line_iter
                        .map(|item| item.split("/").next().unwrap())
                        .collect::<Vec<&str>>();
                    let a = items[0].parse::<usize>().unwrap() - 1;
                    let b = items[1].parse::<usize>().unwrap() - 1;
                    let c = items[2].parse::<usize>().unwrap() - 1;
                    faces.push(vec![a, b, c]);
                }
                _ => {}
            }
        }
        Self { points, faces }
    }

    #[staticmethod]
    fn load_points(points: Vec<Vec<f64>>, faces: Vec<Vec<usize>>) -> Self {
        Self { points, faces }
    }

    fn rotate(&self, angle_x: f64, angle_y: f64, angle_z: f64) -> PyResult<Vec<Vec<f64>>> {
        let mut rotation_matrix = arr2(&[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);

        if angle_x != 0.0 {
            let cos_x = angle_x.cos();
            let sin_x = angle_x.sin();
            rotation_matrix = rotation_matrix.dot(&arr2(&[
                [1.0, 0.0, 0.0],
                [0.0, cos_x, -sin_x],
                [0.0, sin_x, cos_x],
            ]));
        }
        if angle_y != 0.0 {
            let cos_y = angle_y.cos();
            let sin_y = angle_y.sin();
            rotation_matrix = rotation_matrix.dot(&arr2(&[
                [cos_y, 0.0, sin_y],
                [0.0, 1.0, 0.0],
                [-sin_y, 0.0, cos_y],
            ]));
        }
        if angle_z != 0.0 {
            let cos_z = angle_z.cos();
            let sin_z = angle_z.sin();
            rotation_matrix = rotation_matrix.dot(&arr2(&[
                [cos_z, -sin_z, 0.0],
                [sin_z, cos_z, 0.0],
                [0.0, 0.0, 1.0],
            ]));
        }

        Ok(self
            .points
            .par_iter()
            .map(|point_vec| {
                rotation_matrix
                    .dot(&arr2(&[[point_vec[0]], [point_vec[1]], [point_vec[2]]]))
                    .into_raw_vec()
            })
            .collect::<Vec<Vec<f64>>>())
    }

    fn rotate_in_place(&mut self, angle_x: f64, angle_y: f64, angle_z: f64) -> PyResult<()> {
        self.points = self.rotate(angle_x, angle_y, angle_z)?;
        Ok(())
    }

    fn get_view(
        &self,
        origin: Vec<f64>,
        orientation: Vec<f64>,
        focal: Vec<f64>,
    ) -> PyResult<Vec<Vec<f64>>> {
        let sx = orientation[0].sin();
        let cx = orientation[0].cos();
        let sy = orientation[1].sin();
        let cy = orientation[1].cos();
        let sz = orientation[2].sin();
        let cz = orientation[2].cos();
        let ex = focal[0];
        let ey = focal[1];
        let ez = focal[2];
        Ok(self
            .points
            .par_iter()
            .map(|point| {
                let x = point[0] - origin[0];
                let y = point[1] - origin[1];
                let z = point[2] - origin[2];
                let dx = cy * (sz * y + cz * x) - sy * z;
                let dy = sx * (cy * z + sy * (sz * y + cz * x)) + cx * (cz * y - sz * x);
                let dz = cx * (cy * z + sy * (sz * y + cz * x)) - sx * (cz * y - sz * x);
                vec![ez * dx / dz + ex, ez * dy / dz + ey]
            })
            .collect::<Vec<Vec<f64>>>())
    }

    fn get_poly(&self, focal: Vec<f64>, origin: Vec<f64>) -> PyResult<Vec<Vec<Vec<f64>>>> {
        let view = self.get_view(origin, vec![0.0, 0.0, 0.0], focal);
        Ok(self
            .faces
            .par_iter()
            .map(|face| {
                face.par_iter()
                    .map(|index| view.as_ref().unwrap()[*index].clone())
                    .collect::<Vec<Vec<f64>>>()
            })
            .collect::<Vec<Vec<Vec<f64>>>>())
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

    fn get_view(
        &self,
        py: Python<'_>,
        focal: Vec<f64>,
        origin: Vec<f64>,
    ) -> PyResult<Vec<Vec<f64>>> {
        Ok(self
            .shapes
            .iter()
            .map(|shape| {
                shape
                    .borrow(py)
                    .get_view(origin.clone(), vec![0.0, 0.0, 0.0], focal.clone())
                    .unwrap()
            })
            .reduce(|a, b| [a, b].concat())
            .unwrap())
    }

    fn get_poly(
        &self,
        py: Python<'_>,
        focal: Vec<f64>,
        origin: Vec<f64>,
    ) -> PyResult<Vec<Vec<Vec<f64>>>> {
        Ok(self
            .shapes
            .iter()
            .map(|shape| {
                shape
                    .borrow(py)
                    .get_poly(focal.clone(), origin.clone())
                    .unwrap()
            })
            .reduce(|a, b| [a, b].concat())
            .unwrap())
    }
}
