

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
	let simulation = simulation::SimulationWindow::new("Rubiks Cube Simulation")?;
	simulation.window_loop();
	Ok(())
}
