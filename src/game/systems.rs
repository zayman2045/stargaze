//! Contains the systems for controlling the simulation state in the game.

use bevy::prelude::*;

use super::SimulationState;

/// Pauses the game simulation.
pub fn pause_simulation(mut next_simulation_state: ResMut<NextState<SimulationState>>) {
    next_simulation_state.set(SimulationState::Paused);
}

/// Runs the game simulation.
pub fn run_simulation(mut next_simulation_state: ResMut<NextState<SimulationState>>) {
    next_simulation_state.set(SimulationState::Running);
}

/// Toggles the SimulationState between running and paused when the "Space" key is pressed.
pub fn toggle_simulation(
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if State::get(&simulation_state).eq(&SimulationState::Running) {
            next_simulation_state.set(SimulationState::Paused);
        }
        if State::get(&simulation_state).eq(&SimulationState::Paused) {
            next_simulation_state.set(SimulationState::Running);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn did_simulation_pause() {
        // Setup app
        let mut app = App::new();

        // Add SimulationState
        app.add_state::<SimulationState>();

        // Add pause_simulation system
        app.add_systems(Update, pause_simulation);

        // Run systems
        app.update();

        // Check resulting changes
        let next_state = app.world.resource::<NextState<SimulationState>>().0.unwrap();
        assert_eq!(next_state, SimulationState::Paused);
    }

    #[test]
    fn did_simulation_run() {
        // Setup app
        let mut app = App::new();

        // Add SimulationState
        app.add_state::<SimulationState>();

        // Add pause_simulation system
        app.add_systems(Update, run_simulation);

        // Run systems
        app.update();

        // Check resulting changes
        let next_state = app.world.resource::<NextState<SimulationState>>().0.unwrap();
        assert_eq!(next_state, SimulationState::Running);
    }
}