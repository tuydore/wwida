use clap::clap_derive::ValueEnum;
use itertools::Itertools;

use crate::components::{short_string::SHORT_STRING_THRESHOLD, task::Task, time::DATE_FORMAT, TaskId};

struct TickTock(bool);

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum TaskListFormatter {
    Short,
    Long,
}

impl TaskListFormatter {
    pub(crate) fn print(&self, tasks: Vec<(TaskId, &Task)>) {
        match self {
            TaskListFormatter::Short => print_short(tasks.into_iter()),
            TaskListFormatter::Long => print_long(tasks.into_iter()),
        }
    }
}

impl Iterator for TickTock {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = !self.0;
        Some(self.0)
    }
}

impl Default for TaskListFormatter {
    fn default() -> Self {
        Self::Short
    }
}

pub(crate) fn print_short<'t>(tasks: impl Iterator<Item = (TaskId, &'t Task)>) {
    for ((id, task), dash_clock) in tasks.zip(TickTock(false).into_iter()) {
        let short = format!("{} ", task.short);

        if dash_clock {
            print!("[ {:^7} ] {short:─<52}", id);
        } else {
            print!("[ {:^7} ] {short:╌<52}", id);
        }

        if let Some(deadline) = task.deadline.0 {
            print!(" [ DUE {} ]", deadline.format(DATE_FORMAT));
        } else {
            print!(" [ {:^20} ]", "NO DEADLINE");
        }
        print!("[ {:<5} ]", task.priority.as_symbol());

        println!("[ {:^50} ]", task.last_status().to_string())
    }
}

pub(crate) fn print_long<'t>(tasks: impl Iterator<Item = (TaskId, &'t Task)>) {
    for (id, task) in tasks {
        let id_str = format!("ID {id}");
        let sep = std::iter::once('┌')
            .chain(std::iter::repeat('─').take(id_str.len() + 1))
            .join("");
        println!(" {}\n{}", id_str, sep);
        println!("│ SHORT    :: {}", task.short);
        if let Some(long) = &task.long {
            print_split_string("│ LONG     :: ", long, SHORT_STRING_THRESHOLD);
        }
        println!("│ CATEGORY :: {}", task.category);
        if let Some(deadline) = task.deadline.0 {
            println!("│ DEADLINE :: {}", deadline.format(DATE_FORMAT));
        }
        println!(
            "│ STATUS   :: {}",
            task.statuses.last().expect("task should have a last status")
        );
        println!("│ PRIORITY :: {}", task.priority.as_symbol());
        if !task.tags.is_empty() {
            print_split_string("│ TAGS     :: ", &task.tags.iter().join(", "), SHORT_STRING_THRESHOLD);
        }
        println!();
    }
}

fn print_split_string(indent: &str, s: &str, threshold: usize) {
    print!("{indent}");
    let indent = " ".repeat(indent.len() - 3);
    let mut counter = threshold;
    for word in s.split_ascii_whitespace() {
        if counter > word.len() {
            counter -= word.len() + 1;
            print!("{word} ");
        } else {  // TODO: fix edge case of words >= 50 chars
            counter = threshold - word.len() - 1;
            print!("\n│{indent}{word} ");
        }
    }
    println!();
}