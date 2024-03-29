use pyo3::prelude::*;
include!(concat!(env!("OUT_DIR"), "/module.rs"));
use nalgebra::{Matrix3, Rotation3, Vector3};
use rayon::prelude::*;

struct Polygon {
    points: Vec<Vector3<f64>>,
    normal: Vector3<f64>,
}

impl Polygon {
    fn new(points: Vec<Vector3<f64>>) -> Self {
        let a: Vector3<f64> = points[0];
        let b: Vector3<f64> = points[1];
        let c: Vector3<f64> = points[2];

        fn inv_sqrt(number: f64) -> f64 {
            let mut i: i64 = number.to_bits() as i64;
            i = 0x5fe6eb50c7b537a9_i64.wrapping_sub(i >> 1);
            let y = f64::from_bits(i as u64);
            y * (1.5 - (number * 0.5 * y * y))
        }

        // calculate surface normal of the polygon
        // let normal: Vector3<f64> = points.iter().reduce(|a, b| &a.cross(b)).unwrap();
        let normal: Vector3<f64> = (b - a).cross(&(c - a));
        let norm = inv_sqrt(normal.norm_squared());
        Self {
            points,
            normal: normal * norm,
        }
    }
    fn transform_inplace(&mut self, matrix: Matrix3<f64>) {
        for point in self.points.iter_mut() {
            matrix.mul_to(&point.clone(), point);
        }
        matrix.mul_to(&self.normal.clone(), &mut self.normal);
    }
    fn transform(&self, matrix: Matrix3<f64>) -> Self {
        let mut points = self.points.clone();
        for point in points.iter_mut() {
            matrix.mul_to(&point.clone(), point);
        }
        let mut normal = self.normal.clone();
        matrix.mul_to(&self.normal.clone(), &mut normal);
        Self { points, normal }
    }
}

#[pyclass]
struct Mesh {
    polygons: Vec<Polygon>,
}

#[pymethods]
impl Mesh {
    #[new]
    fn load(path: String) -> Self {
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
                    points.push(Vector3::new(x, y, z));
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
        let polygons = faces
            .into_par_iter()
            .map(|face| {
                let points = face
                    .into_iter()
                    .map(|index| points[index].clone())
                    .collect();
                Polygon::new(points)
            })
            .collect();
        Self { polygons }
    }

    fn rotate_in_place(&mut self, angle_x: f64, angle_y: f64, angle_z: f64) -> PyResult<()> {
        let rotation = Rotation3::from_euler_angles(angle_x, angle_y, angle_z);
        self.polygons
            .par_iter_mut()
            .for_each(|polygon| polygon.transform_inplace(rotation.into()));
        Ok(())
    }

    fn rotate(&self, angle_x: f64, angle_y: f64, angle_z: f64) -> PyResult<Vec<Vec<f64>>> {
        let rotation = Rotation3::from_euler_angles(angle_x, angle_y, angle_z);
        let polygons = self
            .polygons
            .par_iter()
            .map(|polygon| polygon.transform(rotation.into()))
            .collect::<Vec<Polygon>>();
        let mut result = Vec::new();
        for polygon in polygons {
            for point in polygon.points {
                result.push(vec![point.x, point.y, point.z]);
            }
        }
        Ok(result)
    }

    #[args(disable_culling = false)]
    fn get_view(
        &self,
        focal: Vec<f64>,
        origin: Vec<f64>,
        disable_culling: bool,
    ) -> PyResult<Vec<Vec<Vec<f64>>>> {
        let origin = Vector3::new(origin[0], origin[1], origin[2]);
        // let orientation = Vector3::new(orientation[0], orientation[1], orientation[2]);
        let focal = Vector3::new(focal[0], focal[1], focal[2]);
        Ok(self
            .polygons
            .par_iter()
            .filter_map(|polygon| {
                if polygon.normal[2] < 0.0 || disable_culling {
                    let mut points = Vec::new();
                    for point in &polygon.points {
                        let mut point = point.clone();
                        point -= &origin;
                        point *= focal[2] / point[2];
                        point += &focal;
                        points.push(vec![point[0], point[1], point[2]]);
                    }
                    Some(points)
                } else {
                    None
                }
            })
            .collect::<Vec<Vec<Vec<f64>>>>())
    }

    #[args(disable_culling = false, disable_occlusion = false)]
    fn get_shaded(
        &self,
        focal: Vec<f64>,
        origin: Vec<f64>,
        disable_culling: bool,
        disable_occlusion: bool,
    ) -> PyResult<Vec<(Vec<Vec<f64>>, f64)>> {
        let origin = Vector3::new(origin[0], origin[1], origin[2]);
        let focal = Vector3::new(focal[0], focal[1], focal[2]);
        let mut culled = self
            .polygons
            .par_iter()
            .filter_map(|polygon| {
                if polygon.normal[2] < 0.0 || disable_culling {
                    let mut points = Vec::new();
                    for point in &polygon.points {
                        let mut point_view = point.clone();
                        point_view -= &origin;
                        point_view *= focal[2] / point_view[2];
                        point_view += &focal;
                        points.push(vec![point_view[0], point_view[1], point[2]]);
                    }
                    Some((
                        points,
                        polygon.normal.dot(&Vector3::new(0.0, 0.0, 1.0)).abs(),
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<(Vec<Vec<f64>>, f64)>>();
        if !disable_occlusion {
            culled.sort_by_cached_key(|(poly, _)| {
                poly.iter().map(|point| -point[2]).sum::<f64>().round() as i64
            });
        }    
        
        Ok(culled)
    }
}
