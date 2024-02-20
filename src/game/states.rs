use bevy::ecs::schedule::States;

/// The possible states of the game simulation.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
    /// The game simulation is currently running.
    #[default]
    Running,
    /// The game simulation is currently paused.
    Paused,
}