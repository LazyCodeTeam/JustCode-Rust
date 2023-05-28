use std::collections::HashMap;

use common_domain::identifiable::Identifiable;

use crate::{into_modification::IntoModification, Personalize};

use super::{
    historical_answer::HistoricalAnswer, modification::Modification,
    personalized_section::PersonalizedSection, task_preview::TaskPreview,
};

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

impl Personalize<PersonalizedSection> for Section {
    fn personalize(
        self,
        correct_historical_answers: &HashMap<String, HistoricalAnswer>,
    ) -> PersonalizedSection {
        let tasks_preview = self
            .tasks_preview
            .into_iter()
            .map(|task_preview| task_preview.personalize(correct_historical_answers))
            .collect();

        PersonalizedSection {
            id: self.id,
            technology_id: self.technology_id,
            position: self.position,
            title: self.title,
            description: self.description,
            image: self.image,
            tasks_preview,
        }
    }
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
