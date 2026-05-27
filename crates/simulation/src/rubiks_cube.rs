use three_d::*;
use crate::cube::Cube;
use crate::rubiks_action::RubiksAction;

const COLOR_EMPTY: Srgba = Srgba::new_opaque(64, 64, 64);
const COLOR_WHITE: Srgba = Srgba::new_opaque(255, 255, 255);
const COLOR_YELLOW: Srgba = Srgba::new_opaque(255, 255, 0);
const COLOR_GREEN: Srgba = Srgba::new_opaque(0, 255, 0);
const COLOR_BLUE: Srgba = Srgba::new_opaque(0, 0, 255);
const COLOR_RED: Srgba = Srgba::new_opaque(255, 0, 0);
const COLOR_ORANGE: Srgba = Srgba::new_opaque(255, 165, 0);

pub struct RubiksCube {
	cubes: [[[Box<Cube>; 3]; 3]; 3],
}

impl RubiksCube {
	pub fn new(context: &Context) -> Self {
		let cubes = std::array::from_fn(|x| {
			std::array::from_fn(|y| {
				std::array::from_fn(|z| {
					let cx = ((x as i32 - 1) * 2) as f32 * 1.02;
					let cy = ((y as i32 - 1) * 2) as f32 * 1.02;
					let cz = ((z as i32 - 1) * 2) as f32 * 1.02;

					Box::new(Cube::new(&context, vec3(cx, cy, cz), [
						if x == 0 { COLOR_ORANGE } else { COLOR_EMPTY },
						if x == 2 { COLOR_RED } else { COLOR_EMPTY },
						if y == 0 { COLOR_YELLOW } else { COLOR_EMPTY },
						if y == 2 { COLOR_WHITE } else { COLOR_EMPTY },
						if z == 0 { COLOR_BLUE } else { COLOR_EMPTY },
						if z == 2 { COLOR_GREEN } else { COLOR_EMPTY },
					]))
				})
			})
		});

		RubiksCube { cubes }
	}

	pub fn rotate_x(&mut self, i: usize, cw: bool) {
		for y in 0..3 {
			for z in 0..3 {
				self.cubes[i][y][z].rotate_x(cw);
			}
		}

		for (f, t) in Self::rotations(cw) {
			self.swap((i, f.0, f.1), (i, t.0, t.1));
		}
	}

	pub fn rotate_y(&mut self, i: usize, cw: bool) {
		for x in 0..3 {
			for z in 0..3 {
				self.cubes[x][i][z].rotate_y(cw);
			}
		}

		for (f, t) in Self::rotations(!cw) {
			self.swap((f.0, i, f.1), (t.0, i, t.1));
		}
	}

	pub fn rotate_z(&mut self, i: usize, cw: bool) {
		for x in 0..3 {
			for y in 0..3 {
				self.cubes[x][y][i].rotate_z(cw);
			}
		}

		for (f, t) in Self::rotations(cw) {
			self.swap((f.0, f.1, i), (t.0, t.1, i));
		}
	}

	pub fn apply(&mut self, action: RubiksAction) {
		match action {
			RubiksAction::Left { prime, wide } =>  { self.rotate_x(0, prime); if wide { self.rotate_x(1, prime); } },
			RubiksAction::Right { prime, wide } => { self.rotate_x(2, !prime); if wide { self.rotate_x(1, !prime); } },
			RubiksAction::Down { prime, wide } =>  { self.rotate_y(0, prime); if wide { self.rotate_y(1, prime); } },
			RubiksAction::Up { prime, wide } =>    { self.rotate_y(2, !prime); if wide { self.rotate_y(1, !prime); } },
			RubiksAction::Back { prime, wide } =>  { self.rotate_z(0, prime); if wide { self.rotate_z(1, prime); } },
			RubiksAction::Front { prime, wide } => { self.rotate_z(2, !prime); if wide { self.rotate_z(1, !prime); } },
			RubiksAction::Middle { prime } =>      { self.rotate_x(1, prime); },
			RubiksAction::Equatorial { prime } =>  { self.rotate_y(1, prime); },
			RubiksAction::Standing { prime } =>    { self.rotate_z(1, prime); },
			RubiksAction::RotateCubeX { prime } => { self.rotate_x(0, prime); self.rotate_x(1, prime); self.rotate_x(2, prime); },
			RubiksAction::RotateCubeY { prime } => { self.rotate_y(0, prime); self.rotate_y(1, prime); self.rotate_y(2, prime); },
			RubiksAction::RotateCubeZ { prime } => { self.rotate_z(0, !prime); self.rotate_z(1, !prime); self.rotate_z(2, !prime); },
		}
	}

	fn swap(&mut self, index1: (usize, usize, usize), index2: (usize, usize, usize)) {
		if index1 == index2 {
			panic!("Can't swap with itself");
		}

		unsafe {
			let ptr1 = &mut self.cubes[index1.0][index1.1][index1.2] as *mut _;
			let ptr2 = &mut self.cubes[index2.0][index2.1][index2.2] as *mut _;
			std::mem::swap(&mut *ptr1, &mut *ptr2);
		}
	}

	const fn rotations(cw: bool) -> [((usize, usize), (usize, usize)); 6] {
		let mut rotations: [((usize, usize), (usize, usize)); 6] = [
			((0, 2), (0, 0)),
			((0, 0), (2, 0)),
			((2, 0), (2, 2)),
			((0, 1), (1, 0)),
			((1, 0), (2, 1)),
			((2, 1), (1, 2)),
		];

		if !cw {
			rotations.reverse();
		}

		rotations
	}
}

// God forbid allocations
impl<'a> IntoIterator for &'a RubiksCube {
	type Item = <&'a Cube as IntoIterator>::Item;
	type IntoIter = std::iter::FlatMap<
		std::iter::Flatten<
			std::iter::Flatten<
				std::slice::Iter<'a, [[Box<Cube>; 3]; 3]>
			>
		>,
		<&'a Cube as IntoIterator>::IntoIter,
		for<'c> fn(&'c Box<Cube>) -> <&'c Cube as IntoIterator>::IntoIter
	>;

	fn into_iter(self) -> Self::IntoIter {
		self.cubes
			.iter()
			.flatten()
			.flatten()
			.flat_map(|c| c.into_iter())
	}
}
