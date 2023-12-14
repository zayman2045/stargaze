use bevy::prelude::*;

use super::SimulationState;

// Pause the game
pub fn pause_simulation(mut next_simulation_state: ResMut<NextState<SimulationState>>) {
    next_simulation_state.set(SimulationState::Paused);
}

// Run the game
pub fn resume_simulation(mut next_simulation_state: ResMut<NextState<SimulationState>>) {
    next_simulation_state.set(SimulationState::Running);
}

// Toggle the SimulationState between running and paused when the "Space" key pressed.
pub fn toggle_simulation(
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if State::get(&simulation_state).eq(&SimulationState::Running){
            next_simulation_state.set(SimulationState::Paused);
            println!("Simulation Paused.");
        }
        if State::get(&simulation_state).eq(&SimulationState::Paused) {
            next_simulation_state.set(SimulationState::Running);
            println!("Simulation Running.");
        }
    }
}
