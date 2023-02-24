use super::{section::Section, task::Task, technology::Technology};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Modification {
    AddTechnology(Technology),
    AddSection(Section),
    AddTask(Task),
    RemoveTechnology(Technology),
    RemoveSection(Section),
    RemoveTask(Task),
    UpdateTechnology(Technology),
    UpdateSection(Section),
    UpdateTask(Task),
}
