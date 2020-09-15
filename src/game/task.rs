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
pub struct Task {
    info: TaskInfo,
    minigame: Box<dyn Minigame>,
}

impl Task {
    /// Create a new task.
    ///
    /// This uses the minigame's default implementation
    pub fn new<T>(name: String) -> Task 
    where T: Minigame + Default + 'static {
        Task::new_with(name, T::default())
    }

    /// Create a new task from a [`Minigame`] type.
    pub fn new_with<T>(name: String, minigame: T) -> Task
    where T: Minigame + 'static {
        Task {
            info: TaskInfo {
                name,
                completion: 0.0,
            },
            minigame: Box::new(minigame),
        }
    }

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

/// A task pool.
///
/// This pool gaurentees that task IDs will stay consistent until the pool is
/// cleared, i.e. at the end of a game.
pub struct TaskPool {
    pool: Vec<Option<Task>>,
}

impl TaskPool {
    /// Index into the `TaskPool` immutably.
    pub fn get(&self, id: usize) -> Option<&Task> {
        self.pool.get(id)?.into()
    }

    /// Index into the `TaskPool` mutably.
    pub fn get_mut(&mut self, id: usize) -> Option<&mut Task> {
        self.pool.get_mut(id)?.into()
    }

    /// Insert a task with an id.
    ///
    /// Returns the old task that was in its place, or `None` if there was no
    /// task in its place.
    pub fn insert_with(&mut self, task: Task, id: usize) -> Option<Task> {
        if id >= self.pool.len() {
            // allocate memory to add the Task
            self.pool.extend((0..=(self.pool.len() - id)).map(|_| None))
        }

        std::mem::replace(&mut self.pool[id], Some(task))
    }

    /// Insert at the first empty task id.
    ///
    /// Returns the id of the new task. 
    pub fn insert(&mut self, task: Task) -> usize {
        let id = self.first_empty();

        self.insert_with(task, id);
        id
    }

    /// Clears the pool.
    pub fn clear(&mut self) {
        self.pool.iter_mut().for_each(|t| *t = None);
    }

    fn first_empty(&self) -> usize {
        match self.pool.iter().enumerate().find(|(_, t)| t.is_none()) {
            Some((id, _)) => id,
            None => self.pool.len(),
        }
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
