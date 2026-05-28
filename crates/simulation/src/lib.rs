mod cube;
mod rubiks_action;
mod rubiks_cube;
mod utils;

use rubiks_action::RubiksAction;
use rubiks_cube::RubiksCube;

use rand::RngExt;
use three_d::*;

struct Simulation {
	context: Context,
	camera: Camera,
	camera_text: Camera,
	control: OrbitControl,

	rubiks: RubiksCube,
	history: Vec::<RubiksAction>,
	past_active_history_item: usize,

	pressed_keys: std::collections::HashSet<Key>,

	text_generator: TextGenerator<'static>,
	axes: Gm<InstancedMesh, ColorMaterial>,
	history_text_left: Gm<Mesh, ColorMaterial>,
	history_text_right: Gm<Mesh, ColorMaterial>,

	show_axes: bool,
}

impl Simulation {
	fn render(&mut self, frame_input: &mut FrameInput) -> FrameOutput {
		self.camera.set_viewport(frame_input.viewport);
		self.control.handle_events(&mut self.camera, &mut frame_input.events);

		for event in &frame_input.events {
			self.handle_event(&event);
		}

		frame_input
			.screen()
			.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
			.render(
				&self.camera,
				self.rubiks.into_iter(),
				&[],
			);

		if self.show_axes {
			frame_input
				.screen()
				.render(
					&self.camera,
					self.axes.into_iter(),
					&[],
				);
		}

		frame_input
			.screen()
			.render(
				&self.camera_text,
				[&self.history_text_left, &self.history_text_right],
				&[],
			);

		FrameOutput::default()
	}

	fn handle_keypress_event(&mut self, event: &Event) {
		let key = if let Event::KeyPress { kind, .. } = event {
			kind
		} else {
			panic!("handle_keypress_event");
		};

		if !self.pressed_keys.insert(*key) {
			return;
		}

		match event {
			Event::KeyPress { kind: Key::L, modifiers, .. } => {
				self.add_new_action(RubiksAction::Left{ prime: modifiers.shift, wide: modifiers.alt });
			},
			Event::KeyPress { kind: Key::R, modifiers, .. } => {
				self.add_new_action(RubiksAction::Right{ prime: modifiers.shift, wide: modifiers.alt });
			},
			Event::KeyPress { kind: Key::D, modifiers, .. } => {
				self.add_new_action(RubiksAction::Down{ prime: modifiers.shift, wide: modifiers.alt });
			},
			Event::KeyPress { kind: Key::U, modifiers, .. } => {
				self.add_new_action(RubiksAction::Up{ prime: modifiers.shift, wide: modifiers.alt });
			},
			Event::KeyPress { kind: Key::B, modifiers, .. } => {
				self.add_new_action(RubiksAction::Back{ prime: modifiers.shift, wide: modifiers.alt });
			},
			Event::KeyPress { kind: Key::F, modifiers, .. } => {
				self.add_new_action(RubiksAction::Front{ prime: modifiers.shift, wide: modifiers.alt });
			},
			Event::KeyPress { kind: Key::M, modifiers, .. } => {
				self.add_new_action(RubiksAction::Middle{ prime: modifiers.shift });
			},
			Event::KeyPress { kind: Key::E, modifiers, .. } => {
				self.add_new_action(RubiksAction::Equatorial{ prime: modifiers.shift });
			},
			Event::KeyPress { kind: Key::S, modifiers, .. } => {
				self.add_new_action(RubiksAction::Standing{ prime: modifiers.shift });
			},
			Event::KeyPress { kind: Key::X, modifiers, .. } => {
				self.add_new_action(RubiksAction::RotateCubeX{ prime: modifiers.shift });
			},
			Event::KeyPress { kind: Key::Y, modifiers, .. } => {
				self.add_new_action(RubiksAction::RotateCubeY{ prime: modifiers.shift });
			},
			Event::KeyPress { kind: Key::Z, modifiers, .. } => {
				self.add_new_action(RubiksAction::RotateCubeZ{ prime: modifiers.shift });
			},
			Event::KeyPress { kind: Key::Tab, .. } => {
				self.show_axes = !self.show_axes;
			},
			Event::KeyPress { kind: Key::ArrowLeft, modifiers, .. } => {
				if modifiers.ctrl {
					while self.try_move_history_back() {}
				} else {
					self.try_move_history_back();
				}
			},
			Event::KeyPress { kind: Key::ArrowRight, modifiers, .. } => {
				if modifiers.ctrl {
					while self.try_move_history_forward() {}
				} else {
					self.try_move_history_forward();
				}
			},
			Event::KeyPress { kind: Key::F1, .. } => {
				self.reset_rubiks();
			},
			Event::KeyPress { kind: Key::F2, .. } => {
				self.shuffle(20);
			},
			_ => {},
		}
	}

	fn handle_keyrelease_event(&mut self, event: &Event) {
		let key = if let Event::KeyRelease { kind, .. } = event {
			kind
		} else {
			panic!("handle_keyrelease_event");
		};

		self.pressed_keys.remove(key);
	}

	fn handle_event(&mut self, event: &Event) {
		match event {
			Event::KeyPress { .. } => self.handle_keypress_event(event),
			Event::KeyRelease { .. } => self.handle_keyrelease_event(event),
			_ => {},
		}
	}

	fn try_move_history_back(&mut self) -> bool {
		if self.past_active_history_item > 0 {
			self.rubiks.apply(self.history[self.past_active_history_item - 1].inverse());
			self.past_active_history_item -= 1;
			self.recreate_history();
			true
		} else {
			false
		}
	}

	fn try_move_history_forward(&mut self) -> bool {
		if self.past_active_history_item < self.history.len() {
			self.past_active_history_item += 1;
			self.rubiks.apply(self.history[self.past_active_history_item - 1]);
			self.recreate_history();
			true
		} else {
			false
		}
	}

	fn reset_rubiks(&mut self) {
		self.history.clear();
		self.past_active_history_item = 0;
		self.recreate_history();
		self.rubiks = RubiksCube::new(&self.context);
	}

	fn shuffle(&mut self, moves_count: usize) {
		let mut rng = rand::rng();

		for _ in 0..moves_count {
			match rng.random_range(0..9) {
				0 => self.rubiks.rotate_x(0, rng.random_bool(0.5)),
				1 => self.rubiks.rotate_x(1, rng.random_bool(0.5)),
				2 => self.rubiks.rotate_x(2, rng.random_bool(0.5)),
				3 => self.rubiks.rotate_y(0, rng.random_bool(0.5)),
				4 => self.rubiks.rotate_y(1, rng.random_bool(0.5)),
				5 => self.rubiks.rotate_y(2, rng.random_bool(0.5)),
				6 => self.rubiks.rotate_z(0, rng.random_bool(0.5)),
				7 => self.rubiks.rotate_z(1, rng.random_bool(0.5)),
				8 => self.rubiks.rotate_z(2, rng.random_bool(0.5)),
				_ => { panic!("shuffle"); },
			}
		}
	}

	fn add_new_action(&mut self, action: RubiksAction) {
		self.history.truncate(self.past_active_history_item);
		self.history.push(action);
		self.rubiks.apply(action);
		self.past_active_history_item += 1;
		self.recreate_history();
	}

	fn recreate_text(&mut self, s: &str, color: Srgba) -> Gm<Mesh, ColorMaterial> {
		let text_mesh = self.text_generator.generate(s, TextLayoutOptions::default());

		let mut text = Gm::new(
			Mesh::new(&self.context, &text_mesh),
			ColorMaterial {
				color,
				..Default::default()
			},
		);
		text.set_transformation(
			Mat4::from_translation(vec3(
				5.0,
				self.camera_text.viewport().height as f32 - 35.0,
				0.0,
			)),
		);

		text
	}

	fn recreate_history(&mut self) {
		let mut text_left = String::new();
		let mut text_right = String::new();

		for (index, item) in self.history.iter().enumerate() {
			let text_ref = if index < self.past_active_history_item { &mut text_left } else { &mut text_right };
			text_ref.push_str(item.to_string_notation(false));
			text_ref.push_str(" ");
		}

		if text_left.as_bytes().len() > 40 {
			self.history_text_left = self.recreate_text(text_left.split_at(text_left.as_bytes().len() - 40).1, Srgba::BLACK);
		} else {
			self.history_text_left = self.recreate_text(&text_left, Srgba::BLACK);
		}

		self.history_text_right = self.recreate_text(&text_right, Srgba::new_opaque(128, 128, 128));

		if !text_left.is_empty() {
			let prev_transform = self.history_text_right.transformation();
			self.history_text_right.set_transformation(Mat4::from_translation(vec3(self.history_text_left.aabb().max().x, 0.0, 0.0)) * prev_transform);
		}
	}
}

pub struct SimulationWindow {
	window: Window,
	simulation: Simulation,
}

impl SimulationWindow {
	pub fn new(window_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
		let window = Window::new(WindowSettings {
				title: window_name.to_string(),
				..Default::default()
			})?;

		let context = window.gl();

		let camera = Camera::new_perspective(
			window.viewport(),
			vec3(9.0, 9.0, 9.0),
			vec3(0.0, 0.0, 0.0),
			vec3(0.0, 1.0, 0.0),
			degrees(60.0),
			0.01,
			1000.0,
		);

		let camera_text = Camera::new_2d(window.viewport());

		let control = OrbitControl::new(camera.target(), 1.0, 100.0);

		let rubiks = RubiksCube::new(&context);

		let axes = utils::my_axes(&context, 0.1, 5.0);

		let text_generator = TextGenerator::new(include_bytes!("assets/OpenSans-Regular.ttf"), 0, 30.0).unwrap();

		let history_text_left = Gm::new(
			Mesh::new(&context, &text_generator.generate("", TextLayoutOptions::default())),
			ColorMaterial {
				color: Srgba::BLACK,
				..Default::default()
			},
		);
		let history_text_right = Gm::new(
			Mesh::new(&context, &text_generator.generate("", TextLayoutOptions::default())),
			ColorMaterial {
				color: Srgba::BLACK,
				..Default::default()
			},
		);

		Ok(SimulationWindow {
			window,
			simulation: Simulation {
				context,
				camera,
				camera_text,
				control,
				rubiks,
				history: Vec::new(),
				past_active_history_item: 0,
				pressed_keys: std::collections::HashSet::new(),
				text_generator,
				axes,
				history_text_left,
				history_text_right,
				show_axes: false,
			},
		})
	}

	pub fn window_loop(self) {
		let SimulationWindow { window, mut simulation } = self;
		window.render_loop(move |mut frame_input| {
			simulation.render(&mut frame_input)
		});
	}
}
