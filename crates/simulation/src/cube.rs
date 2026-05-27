use three_d::*;

pub struct Cube {
	left: Gm<Mesh, ColorMaterial>,
	right: Gm<Mesh, ColorMaterial>,
	down: Gm<Mesh, ColorMaterial>,
	up: Gm<Mesh, ColorMaterial>,
	back: Gm<Mesh, ColorMaterial>,
	front: Gm<Mesh, ColorMaterial>,
}

impl Cube {
	pub fn new(context: &Context, center: Vector3<f32>, colors: [Srgba; 6]) -> Self {
		Cube {
			left:  Self::create_plane(context, center + vec3(-1.0,  0.0,  0.0), Mat4::from_angle_y(degrees(-90.0)), colors[0]),
			right: Self::create_plane(context, center + vec3( 1.0,  0.0,  0.0), Mat4::from_angle_y(degrees(90.0)), colors[1]),
			down:  Self::create_plane(context, center + vec3( 0.0, -1.0,  0.0), Mat4::from_angle_x(degrees(90.0)), colors[2]),
			up:    Self::create_plane(context, center + vec3( 0.0,  1.0,  0.0), Mat4::from_angle_x(degrees(-90.0)), colors[3]),
			back:  Self::create_plane(context, center + vec3( 0.0,  0.0, -1.0), Mat4::from_angle_y(degrees(180.0)), colors[4]),
			front: Self::create_plane(context, center + vec3( 0.0,  0.0,  1.0), Mat4::identity(), colors[5]),
		}
	}

	fn create_plane(context: &Context, center: Vector3<f32>, rotation: Matrix4<f32>, color: Srgba) -> Gm<Mesh, ColorMaterial> {
		let plane_mesh = CpuMesh::square();
		let mut material = ColorMaterial::new(
			&context,
			&CpuMaterial {
				albedo: color,
				..Default::default()
			},
		);
		material.render_states.cull = Cull::Back;
		let mut plane = Gm::new(
			Mesh::new(&context, &plane_mesh),
			material,
		);
		plane.set_transformation(Mat4::from_translation(center) * rotation);
		plane
	}

	pub fn rotate_x(&mut self, cw: bool) {
		for side in self {
			let prev = side.transformation();
			side.set_transformation(Mat4::from_angle_x(degrees(if cw { -90.0 } else { 90.0 })) * prev);
		}
	}

	pub fn rotate_y(&mut self, cw: bool) {
		for side in self {
			let prev = side.transformation();
			side.set_transformation(Mat4::from_angle_y(degrees(if cw { -90.0 } else { 90.0 })) * prev);
		}
	}

	pub fn rotate_z(&mut self, cw: bool) {
		for side in self {
			let prev = side.transformation();
			side.set_transformation(Mat4::from_angle_z(degrees(if cw { -90.0 } else { 90.0 })) * prev);
		}
	}
}

impl<'a> IntoIterator for &'a Cube {
	type Item = &'a Gm<Mesh, ColorMaterial>;
	type IntoIter = std::array::IntoIter<Self::Item, 6>;

	fn into_iter(self) -> Self::IntoIter {
		let arr: [Self::Item; 6] = [
			&self.left, &self.right,
			&self.down, &self.up,
			&self.front, &self.back,
		];
		arr.into_iter()
	}
}

impl<'a> IntoIterator for &'a mut Cube {
	type Item = &'a mut Gm<Mesh, ColorMaterial>;
	type IntoIter = std::array::IntoIter<Self::Item, 6>;

	fn into_iter(self) -> Self::IntoIter {
		let arr: [Self::Item; 6] = [
			&mut self.left, &mut self.right,
			&mut self.down, &mut self.up,
			&mut self.front, &mut self.back,
		];
		arr.into_iter()
	}
}
