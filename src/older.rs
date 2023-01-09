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

// let cos_x = angle_x.cos();
        // let sin_x = angle_x.sin();
        // let cos_y = angle_y.cos();
        // let sin_y = angle_y.sin();
        // let cos_z = angle_z.cos();
        // let sin_z = angle_z.sin();

        // let rotation_matrix = arr2(&[
        //     [
        //         cos_y * cos_z,
        //         -cos_x * sin_z + sin_x * sin_y * cos_z,
        //         sin_x * sin_z + cos_x * sin_y * cos_z,
        //     ],
        //     [
        //         cos_y * sin_z,
        //         cos_x * cos_z + sin_x * sin_y * sin_z,
        //         -sin_x * cos_z + cos_x * sin_y * sin_z,
        //     ],
        //     [-sin_y, sin_x * cos_y, cos_x * cos_y],
        // ]);




        // fn get_view_deprecated(&self, focal: f64, origin: Vec<f64>) -> PyResult<Vec<Vec<f64>>> {
        //     let camera = arr2(&[[focal, 0.0, 0.0], [0.0, focal, 0.0], [0.0, 0.0, 1.0]]);
        //     Ok(self
        //         .points
        //         .par_iter()
        //         .map(|point_vec| {
        //             let point = arr2(&[
        //                 [point_vec[0] - origin[0]],
        //                 [point_vec[1] - origin[1]],
        //                 [point_vec[2] - origin[2]],
        //             ]);
        //             let view = camera.dot(&point).into_raw_vec();
        //             vec![view[0], view[1]]
        //         })
        //         .collect::<Vec<Vec<f64>>>())
        // }


        // for line in contents.lines() {
        //     let mut line_iter = line.split_whitespace();
        //     match line_iter.next() {
        //         Some("v") => {
        //             let x = line_iter.next().unwrap().parse::<f64>().unwrap();
        //             let y = line_iter.next().unwrap().parse::<f64>().unwrap();
        //             let z = line_iter.next().unwrap().parse::<f64>().unwrap();
        //             points.push(vec![x, y, z]);
        //         }
        //         Some("f") => {
        //             let a = line_iter.next().unwrap().parse::<usize>().unwrap() - 1;
        //             let b = line_iter.next().unwrap().parse::<usize>().unwrap() - 1;
        //             let c = line_iter.next().unwrap().parse::<usize>().unwrap() - 1;
        //             faces.push(vec![a, b, c]);
        //         }
        //         _ => {}
        //     }
        // }
        // Self { points, faces }

        // fn culling(&self) {
        //     let mut normals = self.get_normals().unwrap();
        //     let mut faces = self.faces.clone();
        //     let mut points = self.points.clone();
        //     let mut new_points = Vec::new();
        //     let mut new_faces = Vec::new();
        //     let mut new_normals = Vec::new();
        //     let mut new_indexes = Vec::new();
        //     // adds the faces that are facing the camera
        //     for i in 0..normals.len() {
        //         if normals[i][2] > 0.0 {
        //             new_faces.push(faces[i].clone());
        //             new_normals.push(normals[i].clone());
        //         }
        //     }
        //     // adds the points that are used by the faces
        //     for i in 0..points.len() {
        //         let mut found = false;
        //         for j in 0..new_faces.len() {
        //             if new_faces[j].contains(&i) {
        //                 found = true;
        //                 break;
        //             }
        //         }
        //         if found {
        //             new_points.push(points[i].clone());
        //             new_indexes.push(i);
        //         }
        //     }
        //     // 
        //     for i in 0..new_faces.len() {
        //         for j in 0..new_faces[i].len() {
        //             new_faces[i][j] = new_indexes.iter().position(|&x| x == new_faces[i][j]).unwrap();
        //         }
        //     }
        //     self.faces = new_faces;
        //     self.points = new_points;
        //     self.normals = new_normals;
        // }