use three_d::Srgba;

pub enum CubeColor {
	None,
	Orange,
	Red,
	Yellow,
	White,
	Blue,
	Green,
}

impl CubeColor {
	pub fn to_srgba(&self) -> Srgba {
		match *self {
			CubeColor::None => Srgba::new_opaque(64, 64, 64),
			CubeColor::Orange => Srgba::new_opaque(255, 165, 0),
			CubeColor::Red => Srgba::new_opaque(255, 0, 0),
			CubeColor::Yellow => Srgba::new_opaque(255, 255, 0),
			CubeColor::White => Srgba::new_opaque(255, 255, 255),
			CubeColor::Blue => Srgba::new_opaque(0, 0, 255),
			CubeColor::Green => Srgba::new_opaque(0, 255, 0),
		}
	}
}

pub struct CubeColors {
	pub left: CubeColor,
	pub right: CubeColor,
	pub down: CubeColor,
	pub up: CubeColor,
	pub back: CubeColor,
	pub front: CubeColor,
}
