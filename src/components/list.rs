use super::{task::Task, TaskId};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub(crate) struct Tasks {
    tasks: Vec<Task>
}

impl Tasks {
    pub(crate) fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub(crate) fn has_id(&self, id: TaskId) -> bool {
        id < self.tasks.len()
    }

    pub(crate) fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub(crate) fn get_task_mut(&mut self, id: TaskId) -> Option<&mut Task> {
        self.tasks.get_mut(id)
    }

    pub(crate) fn get_task_mut_err(&mut self, id: TaskId) -> anyhow::Result<&mut Task> {
        self.get_task_mut(id).ok_or_else(|| anyhow::anyhow!("no task at ID {id}"))
    }

    pub(crate) fn num_tasks(&self) -> usize {
        self.tasks.len()
    }

    pub(crate) fn save(self, filepath: &Path) -> Result<()> {
        let bytes = bson::to_vec(&self)?;
        std::fs::write(filepath, bytes)?;
        Ok(())
    }

    pub(crate) fn load(filepath: &Path) -> Result<Self> {
        if filepath.is_file() {
            let file = std::fs::File::open(filepath)?;
            let tasks = bson::from_reader::<_, Self>(file)?;
            Ok(tasks)
        } else {
            Ok(Self::new())
        }
    }

    pub(crate) fn clear(&mut self) {
        self.tasks.clear();
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (TaskId, &Task)> {
        self.tasks.iter().enumerate()
    }
}

#[cfg(test)]
mod tests {
    use crate::format::{print_long, print_short};

    use super::*;

    fn sample_list() -> Tasks {
        let task_vec = vec![
            Task::from_strings("peel potatoes", Some("documentation"), None, Some("today"))
                .expect("could not create task"),
            Task::from_strings("chop carrots", Some("task"), None, Some("today")).expect("could not create task"),
            Task::from_strings(
                "turn on the hob",
                Some("other"),
                Some("really running out of ideas here"),
                Some("tomorrow"),
            )
            .expect("could not create task"),
            Task::from_strings("eat", Some("message"), None, None).expect("could not create task"),
        ];
        Tasks { tasks: task_vec }
    }

    #[test]
    fn display() {
        let list = sample_list();
        print_short(list.tasks.iter());
        println!();
        print_long(list.tasks.iter().enumerate());
    }
}
