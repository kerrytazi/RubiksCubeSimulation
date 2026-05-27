mod consts;
mod cube;
mod rubiks_action;
mod rubiks_cube;
mod simulation;
mod utils;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
	let simulation = simulation::SimulationWindow::new("Rubiks Cube Simulation", (800, 800))?;
	simulation.window_loop();
	Ok(())
}
