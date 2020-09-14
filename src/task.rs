//! Tasks are one of the main gameplay elements of Among Us. They are
//! fundamentally different to "[`Minigame`]s", which actually drive the tasks.
//! Tasks hold information about the name of the task, how many steps there are
//! and where to do the tasks.
//!
//! Most tasks are counted towards the task bar at the upper-right corner.
//! Some tasks are marked as `urgent`, and are displayed with a flashing red
//! and yellow symbol and text. Besides the initialization, tasks simply need
//! to sync their completion unless more functionality is desired.
//!
//! The spawned `Minigame`s control their task parents. The tasks implement no
//! functionality themselves, except for helper functions for networking.

use crate::game::State;

/// A task.
pub struct Task {
    info: TaskInfo,
    minigame: Box<dyn Minigame>,
}

impl Task {
    /// The name of the task.
    pub fn name(&self) -> &str {
        &self.info.name
    }

    /// The completion rate of the task.
    ///
    /// If the completion rate is 1, then the task is considered complete.
    pub fn completion(&self) -> f32 {
        self.info.completion
    }

    /// Begin the task, initializing the minigame.
    pub fn begin(&mut self, state: State) {
        self.minigame.begin(state, &mut self.info);
    }
}

/// Task information.
pub struct TaskInfo {
    pub name: String,
    pub completion: f32,
}

/// Minigame controller.
///
/// These structs are instantiated once and persist data as long as the same
/// task is active. The [`Minigame::begin()`] callback should be used to reset 
/// any minigame states in the case of a backout, or not if you wish.
pub trait Minigame {
    /// Start a minigame.
    ///
    /// This is called when the user access a unit to begin the minigame, and
    /// before the minigame is displayed to the user on the screen.
    fn begin(&mut self, state: State, task: &mut TaskInfo);
}
