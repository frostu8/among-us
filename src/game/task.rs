//! Task interactions.
//!
//! Tasks are one of the main gameplay elements of Among Us. They are
//! fundamentally different to "[`Minigame`]s", which actually drive the tasks.
//! Tasks hold information about the name of the task, how many steps there are
//! and where to do the tasks.
//!
//! Most tasks are counted towards the task bar at the upper-right corner. Some
//! tasks use a timer construct, and are displayed with flashing yellow and red
//! colors. Besides the initialization, tasks simply need to sync their 
//! completion unless more functionality is desired.
//!
//! Tasks are actually part of a task pool detailing all of the tasks in a
//! single game. This not only allows clients to be more efficient with their
//! data, but this allows clients to share tasks.
//!
//! The spawned `Minigame`s control their task parents. The tasks implement no
//! functionality themselves, except for helper functions for networking.

use crate::game::State;

/// A task.
///
/// This is purely the data part of a task.
pub struct Task {
    name: String,
}

impl Task {
    /// The name of the task.
    pub fn name(&self) -> &str {
        &self.name
    }
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
    ///
    /// Never called on the server.
    fn begin(&mut self, state: State, task: &mut TaskInfo);
}
