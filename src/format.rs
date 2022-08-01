use clap::clap_derive::ArgEnum;
use itertools::Itertools;

use crate::components::{short_string::SHORT_STRING_THRESHOLD, task::Task, time::DATE_FORMAT, TaskId};

struct TickTock(bool);

#[derive(Debug, Clone, ArgEnum)]
pub(crate) enum TaskListFormatter {
    Short,
    Long,
}

impl TaskListFormatter {
    pub(crate) fn print<'t>(&self, tasks: impl Iterator<Item = (TaskId, &'t Task)>) {
        match self {
            TaskListFormatter::Short => print_short(tasks.map(|(_, task)| task)),
            TaskListFormatter::Long => print_long(tasks),
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

pub(crate) fn print_short<'t>(tasks: impl Iterator<Item = &'t Task>) {
    for (task, dash_clock) in tasks.zip(TickTock(false).into_iter()) {
        let short = format!("{} ", task.short);

        if dash_clock {
            print!("[{:^15}] {short:─<51}", task.category.to_string());
        } else {
            print!("[{:^15}] {short:╌<51}", task.category.to_string());
        }

        if let Some(deadline) = task.deadline {
            println!(" [ DUE {} ]", deadline.format(DATE_FORMAT));
        } else {
            println!(" [{:^22}]", "NO DEADLINE");
        }
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
            print!("│ LONG     :: ");
            let mut counter = SHORT_STRING_THRESHOLD ;
            for word in long.split_ascii_whitespace() {
                if counter > word.len() {
                    counter -= word.len() + 1;
                    print!("{word} ");
                } else { // TODO: fix edge case of words >= 50 chars
                    counter = SHORT_STRING_THRESHOLD - word.len() - 1;
                    print!("\n│             {word} ");
                }
            }
            println!();
        }
        println!("│ CATEGORY :: {}", task.category);
        if let Some(deadline) = task.deadline {
            println!("│ DEADLINE :: {}", deadline.format(DATE_FORMAT));
        }
        println!(
            "│ STATUS   :: {}",
            task.statuses.last().expect("task should have a last status")
        );
        println!();
    }
}
