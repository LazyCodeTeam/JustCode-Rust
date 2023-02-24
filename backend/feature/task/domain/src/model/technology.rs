use common_domain::identifiable::Identifiable;

use crate::into_modification::IntoModification;

use super::{modification::Modification, section_preview::SectionPreview};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Technology {
    pub id: String,
    pub name: String,
    pub position: u64,
    pub description: Option<String>,
    pub image: Option<String>,
    pub sections_preview: Vec<SectionPreview>,
}

impl IntoModification for Technology {
    fn into_add_modification(self) -> Modification {
        Modification::AddTechnology(self)
    }

    fn into_remove_modification(self) -> Modification {
        Modification::RemoveTechnology(self)
    }

    fn into_update_modification(self) -> Modification {
        Modification::UpdateTechnology(self)
    }
}

impl Identifiable for Technology {
    type Id = String;

    fn id(&self) -> &String {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_modification() {
        let technology = Technology {
            id: "id".to_string(),
            name: "name".to_string(),
            position: 0,
            description: None,
            image: None,
            sections_preview: vec![],
        };

        assert_eq!(
            technology.clone().into_add_modification(),
            Modification::AddTechnology(technology.clone())
        );
        assert_eq!(
            technology.clone().into_remove_modification(),
            Modification::RemoveTechnology(technology.clone())
        );
        assert_eq!(
            technology.clone().into_update_modification(),
            Modification::UpdateTechnology(technology)
        );
    }

    #[test]
    fn id() {
        let technology = Technology {
            id: "id".to_string(),
            name: "name".to_string(),
            description: None,
            position: 0,
            image: None,
            sections_preview: vec![],
        };

        assert_eq!(technology.id(), "id");
    }
}
