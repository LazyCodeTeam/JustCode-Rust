use common_domain::identifiable::Identifiable;

use crate::into_modification::IntoModification;

use super::{modification::Modification, task_content::TaskContent};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Task {
    pub id: String,
    pub section_id: String,
    pub position: u64,
    pub title: String,
    pub difficulty: u8,
    pub dynamic: bool,
    pub for_anonymous: bool,
    pub content: TaskContent,
}

impl IntoModification for Task {
    fn into_add_modification(self) -> Modification {
        Modification::AddTask(self)
    }

    fn into_remove_modification(self) -> Modification {
        Modification::RemoveTask(self)
    }

    fn into_update_modification(self) -> Modification {
        Modification::UpdateTask(self)
    }
}

impl Identifiable for Task {
    type Id = String;

    fn id(&self) -> &String {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_modification() {
        let task = Task {
            id: "id".to_owned(),
            section_id: "section_id".to_owned(),
            title: "title".to_owned(),
            position: 0,
            difficulty: 1,
            dynamic: true,
            for_anonymous: false,
            content: TaskContent::Empty,
        };

        assert_eq!(
            task.clone().into_add_modification(),
            Modification::AddTask(task.clone())
        );
        assert_eq!(
            task.clone().into_remove_modification(),
            Modification::RemoveTask(task.clone())
        );
        assert_eq!(
            task.clone().into_update_modification(),
            Modification::UpdateTask(task)
        );
    }

    #[test]
    fn test_id() {
        let task = Task {
            id: "id".to_owned(),
            section_id: "section_id".to_owned(),
            title: "title".to_owned(),
            difficulty: 1,
            position: 0,
            dynamic: true,
            for_anonymous: false,
            content: TaskContent::Empty,
        };

        assert_eq!(task.id(), &"id".to_owned());
    }
}
