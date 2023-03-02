use std::collections::HashMap;

use common_domain::identifiable::Identifiable;

use crate::{into_modification::IntoModification, model::modification::Modification};

pub trait DetectModifications {
    fn detect_modifications(self, expected: Self) -> Vec<Modification>;
}

impl<T> DetectModifications for Vec<T>
where
    T: Identifiable<Id = String> + IntoModification + PartialEq,
{
    fn detect_modifications(self, expected: Self) -> Vec<Modification> {
        let mut expected_by_id = expected
            .into_iter()
            .map(|expected| (expected.id().to_owned(), expected))
            .collect::<HashMap<String, T>>();

        let mut modifications = vec![];

        for actual in self.into_iter() {
            if let Some(expected) = expected_by_id.remove(actual.id()) {
                if let Some(modification) = detect_modification(actual, expected) {
                    modifications.push(modification);
                }
            } else {
                modifications.push(actual.into_remove_modification());
            }
        }

        modifications.extend(
            expected_by_id
                .into_values()
                .map(|expected| expected.into_add_modification()),
        );

        modifications
    }
}

fn detect_modification<T>(actual: T, expected: T) -> Option<Modification>
where
    T: PartialEq + IntoModification,
{
    if actual != expected {
        Some(expected.into_update_modification())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::technology::Technology;

    #[test]
    fn detect_modifications_no_modification() {
        let actual = vec![Technology {
            id: "id".to_string(),
            name: "name".to_string(),
            position: 0,
            description: None,
            image: None,
            sections_preview: vec![],
        }];

        let expected = vec![Technology {
            id: "id".to_string(),
            name: "name".to_string(),
            position: 0,
            description: None,
            image: None,
            sections_preview: vec![],
        }];

        assert_eq!(actual.detect_modifications(expected), vec![]);
    }

    #[test]
    fn detect_modifications_with_modification() {
        let actual = vec![Technology {
            id: "id".to_string(),
            name: "name".to_string(),
            position: 0,
            description: None,
            image: None,
            sections_preview: vec![],
        }];

        let expected = vec![Technology {
            id: "id".to_string(),
            name: "name".to_string(),
            position: 0,
            description: Some("description".to_string()),
            image: None,
            sections_preview: vec![],
        }];

        assert_eq!(
            actual.detect_modifications(expected),
            vec![Modification::UpdateTechnology(Technology {
                id: "id".to_string(),
                name: "name".to_string(),
                description: Some("description".to_string()),
                position: 0,
                image: None,
                sections_preview: vec![],
            })]
        );
    }

    #[test]
    fn detect_modifications_with_remove() {
        let actual = vec![Technology {
            id: "id".to_string(),
            name: "name".to_string(),
            description: Some("description".to_string()),
            position: 0,
            image: None,
            sections_preview: vec![],
        }];

        let expected = vec![];

        assert_eq!(
            actual.detect_modifications(expected),
            vec![Modification::RemoveTechnology(Technology {
                id: "id".to_string(),
                name: "name".to_string(),
                description: Some("description".to_string()),
                position: 0,
                image: None,
                sections_preview: vec![],
            })]
        );
    }

    #[test]
    fn detect_modifications_with_add() {
        let actual = vec![];

        let expected = vec![Technology {
            id: "id".to_string(),
            name: "name".to_string(),
            description: Some("description".to_string()),
            position: 0,
            image: None,
            sections_preview: vec![],
        }];

        assert_eq!(
            actual.detect_modifications(expected),
            vec![Modification::AddTechnology(Technology {
                id: "id".to_string(),
                name: "name".to_string(),
                description: Some("description".to_string()),
                position: 0,
                image: None,
                sections_preview: vec![],
            })]
        );
    }
}
