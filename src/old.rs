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
                vec![ez * dx / dz + ex, ez * dy / dz + ey, -point[2]]
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

    fn get_normals(&self) -> PyResult<Vec<Vec<f64>>> {
        fn inv_sqrt(number: f32) -> f32 {
            let mut i: i32 = number.to_bits() as i32;
            i = 0x5F375A86_i32.wrapping_sub(i >> 1);
            let y = f32::from_bits(i as u32);
            y * (1.5 - (number * 0.5 * y * y))
        }
        Ok(self
            .faces
            .par_iter()
            .map(|face| {
                let a = self.points[face[0]].clone();
                let b = self.points[face[1]].clone();
                let c = self.points[face[2]].clone();
                let ab = vec![b[0] - a[0], b[1] - a[1], b[2] - a[2]];
                let ac = vec![c[0] - a[0], c[1] - a[1], c[2] - a[2]];
                let normal = vec![
                    ab[1] * ac[2] - ab[2] * ac[1],
                    ab[2] * ac[0] - ab[0] * ac[2],
                    ab[0] * ac[1] - ab[1] * ac[0],
                ];
                let norm = inv_sqrt(
                    (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]) as f32,
                ) as f64;
                vec![normal[0] * norm, normal[1] * norm, normal[2] * norm]
            })
            .collect::<Vec<Vec<f64>>>())
    }

    fn get_lighting(&self, light: Vec<f64>) -> PyResult<Vec<f64>> {
        let normals = self.get_normals()?;
        Ok(normals
            .par_iter()
            .map(|normal| {
                let dot = normal[0] * light[0] + normal[1] * light[1] + normal[2] * light[2];
                if dot < 0.0 {
                    0.0
                } else {
                    dot
                }
            })
            .collect::<Vec<f64>>())
    }

    fn get_culled(
        &self,
        focal: Vec<f64>,
        origin: Vec<f64>,
    ) -> PyResult<Vec<(Vec<Vec<f64>>, Vec<f64>)>> {
        let poly = self.get_poly(focal, origin)?;
        let normals = self.get_normals()?;

        let mut culled = (0..normals.len())
            .into_par_iter()
            .filter(|index| normals[*index][2] < 0.0)
            .map(|index| (poly[index].clone(), normals[index].clone()))
            .collect::<Vec<(Vec<Vec<f64>>, Vec<f64>)>>();

        culled.sort_by_cached_key(|(poly, _)| {
            poly.iter()
                .map(|point| point[2])
                .sum::<f64>()
                // .reduce(f64::max)
                // .unwrap()
                .round() as i64
        });
        Ok(culled)
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
