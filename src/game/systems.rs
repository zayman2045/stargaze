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

        // Initialize the necessary states and events
        app.add_state::<SimulationState>();
        *app.world.resource_mut::<State<SimulationState>>() = State::new(SimulationState::Running);

        // Add systems
        app.add_systems(Update, pause_simulation);

        // Run systems
        app.update();

        // Check resulting changes
        let next_state = app
            .world
            .resource::<NextState<SimulationState>>()
            .0
            .unwrap();
        assert_eq!(next_state, SimulationState::Paused);
    }

    #[test]
    fn did_simulation_run() {
        // Setup app
        let mut app = App::new();

        // Initialize the necessary states and events
        app.add_state::<SimulationState>();
        *app.world.resource_mut::<State<SimulationState>>() = State::new(SimulationState::Paused);

        // Add systems
        app.add_systems(Update, run_simulation);

        // Run systems
        app.update();

        // Check resulting changes
        let next_state = app
            .world
            .resource::<NextState<SimulationState>>()
            .0
            .unwrap();
        assert_eq!(next_state, SimulationState::Running);
    }

    #[test]
    fn toggle_simulation_from_running_to_paused() {
        // Setup app
        let mut app = App::new();

        // Initialize the necessary states and events
        app.add_state::<SimulationState>();
        *app.world.resource_mut::<State<SimulationState>>() = State::new(SimulationState::Running);

        // Insert keyboard input resource with the "Space" key just pressed
        let mut keyboard_input = Input::<KeyCode>::default();
        keyboard_input.press(KeyCode::Space);
        app.insert_resource(keyboard_input);

        // Add systems
        app.add_systems(Update, toggle_simulation);

        // Run systems
        app.update();

        // Check resulting changes
        let next_state = app
            .world
            .resource::<NextState<SimulationState>>()
            .0
            .unwrap();
        assert_eq!(
            next_state,
            SimulationState::Paused,
            "The simulation state should toggle to Paused after pressing Space."
        );
    }

    #[test]
    fn toggle_simulation_from_paused_to_running() {
        // Setup app
        let mut app = App::new();

        // Initialize the necessary states and events
        app.add_state::<SimulationState>();
        *app.world.resource_mut::<State<SimulationState>>() = State::new(SimulationState::Paused);

        // Insert keyboard input resource with the "Space" key just pressed
        let mut keyboard_input = Input::<KeyCode>::default();
        keyboard_input.press(KeyCode::Space);
        app.insert_resource(keyboard_input);

        // Add systems
        app.add_systems(Update, toggle_simulation);

        // Run systems
        app.update();

        // Check resulting changes
        let next_state = app
            .world
            .resource::<NextState<SimulationState>>()
            .0
            .unwrap();
        assert_eq!(
            next_state,
            SimulationState::Running,
            "The simulation state should toggle to Running after pressing Space."
        );
    }
}
