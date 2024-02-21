//! Contains the systems related to the score in the game.

use super::resources::*;
use bevy::prelude::*;

/// Inserts the Score Resource into the world.
pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default())
}

/// Removes the Score Resource from the world.
pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_score_adds_resource_to_world() {
        // Setup app
        let mut app = App::new();

        // Add systems
        app.add_systems(Update, insert_score);

        // Verify resource was not previously present
        assert!(!app.world.contains_resource::<Score>());

        // Run startup systems to insert the Score resource
        app.update();

        // Check resulting changes
        assert!(app.world.contains_resource::<Score>());
    }

    #[test]
    fn remove_score_removes_resource_from_world() {
        // Setup app
        let mut app = App::new();

        // Insert Score resource
        app.insert_resource(Score::default());

        // Verify resource is present
        assert!(app.world.contains_resource::<Score>());

        // Add systems
        app.add_systems(Update, remove_score);

        // Run startup systems to remove the Score resource
        app.update();

        // Check resulting changes
        assert!(!app.world.contains_resource::<Score>());
    }
}
