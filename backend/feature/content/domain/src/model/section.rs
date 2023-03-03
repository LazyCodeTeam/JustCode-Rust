use common_domain::identifiable::Identifiable;

use crate::into_modification::IntoModification;

use super::{modification::Modification, task_preview::TaskPreview};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Section {
    pub id: String,
    pub technology_id: String,
    pub position: u64,
    pub title: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub tasks_preview: Vec<TaskPreview>,
}

impl IntoModification for Section {
    fn into_add_modification(self) -> Modification {
        Modification::AddSection(self)
    }

    fn into_remove_modification(self) -> Modification {
        Modification::RemoveSection(self)
    }

    fn into_update_modification(self) -> Modification {
        Modification::UpdateSection(self)
    }
}

impl Identifiable for Section {
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
        let section = Section {
            id: "id".to_owned(),
            technology_id: "technology_id".to_owned(),
            title: "title".to_owned(),
            position: 0,
            description: Some("description".to_owned()),
            image: Some("image".to_owned()),
            tasks_preview: vec![],
        };

        assert_eq!(
            section.clone().into_add_modification(),
            Modification::AddSection(section.clone())
        );
        assert_eq!(
            section.clone().into_remove_modification(),
            Modification::RemoveSection(section.clone())
        );
        assert_eq!(
            section.clone().into_update_modification(),
            Modification::UpdateSection(section)
        );
    }

    #[test]
    fn test_identifiable() {
        let section = Section {
            id: "id".to_owned(),
            technology_id: "technology_id".to_owned(),
            title: "title".to_owned(),
            position: 0,
            description: Some("description".to_owned()),
            image: Some("image".to_owned()),
            tasks_preview: vec![],
        };

        assert_eq!(section.id(), "id");
    }
}
