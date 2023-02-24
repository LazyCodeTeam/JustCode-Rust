use super::{
    expected_technology_data::ExpectedTechnologyData, modification::Modification, section::Section,
    task::Task, technology::Technology,
};
use crate::detect_modifications::DetectModifications;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RawTasksTree {
    pub technologies: Vec<Technology>,
    pub sections: Vec<Section>,
    pub tasks: Vec<Task>,
}

impl RawTasksTree {
    pub fn detect_changes(self, expected: Vec<ExpectedTechnologyData>) -> Vec<Modification> {
        let expected: RawTasksTree = expected.into();
        let technology_changes = self
            .technologies
            .detect_modifications(expected.technologies);
        let section_changes = self.sections.detect_modifications(expected.sections);
        let task_changes = self.tasks.detect_modifications(expected.tasks);

        technology_changes
            .into_iter()
            .chain(section_changes.into_iter())
            .chain(task_changes.into_iter())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::model::{
        expected_section_data::ExpectedSectionData, expected_task_data::ExpectedTaskData,
        section_preview::SectionPreview, task_content::TaskContent, task_preview::TaskPreview,
    };

    use super::*;

    #[test]
    fn detect_changes_changes() {
        let actual = RawTasksTree {
            technologies: vec![Technology {
                id: "technology_id".to_owned(),
                name: "name".to_owned(),
                description: Some("description".to_owned()),
                position: 0,
                image: Some("image".to_owned()),
                sections_preview: vec![SectionPreview {
                    id: "section_id".to_owned(),
                    title: "title".to_owned(),
                }],
            }],
            sections: vec![Section {
                id: "section_id".to_owned(),
                technology_id: "technology_id".to_owned(),
                title: "title".to_owned(),
                position: 0,
                description: Some("description".to_owned()),
                image: Some("image".to_owned()),
                tasks_preview: vec![TaskPreview {
                    id: "task_id".to_owned(),
                    title: "title".to_owned(),
                }],
            }],
            tasks: vec![Task {
                id: "task_id".to_owned(),
                section_id: "section_id".to_owned(),
                title: "title".to_owned(),
                difficulty: 1,
                position: 0,
                dynamic: false,
                for_anonymous: false,
                content: TaskContent::Empty,
            }],
        };

        let expected = vec![ExpectedTechnologyData {
            id: "technology_id".to_owned(),
            name: "name".to_owned(),
            description: Some("description".to_owned()),
            image: Some("image".to_owned()),
            sections: vec![ExpectedSectionData {
                id: "section_id".to_owned(),
                title: "title".to_owned(),
                description: Some("description".to_owned()),
                image: Some("image".to_owned()),
                tasks: vec![ExpectedTaskData {
                    id: "task_id".to_owned(),
                    title: "title".to_owned(),
                    difficulty: 1,
                    dynamic: false,
                    for_anonymous: false,
                    content: TaskContent::Empty,
                }],
            }],
        }];

        assert_eq!(actual.detect_changes(expected), vec![]);
    }

    #[test]
    fn detect_changes_additions() {
        let actual = RawTasksTree {
            technologies: vec![],
            sections: vec![],
            tasks: vec![],
        };

        let expected = vec![ExpectedTechnologyData {
            id: "technology_id".to_owned(),
            name: "name".to_owned(),
            description: Some("description".to_owned()),
            image: Some("image".to_owned()),
            sections: vec![ExpectedSectionData {
                id: "section_id".to_owned(),
                title: "title".to_owned(),
                description: Some("description".to_owned()),
                image: Some("image".to_owned()),
                tasks: vec![ExpectedTaskData {
                    id: "task_id".to_owned(),
                    title: "title".to_owned(),
                    difficulty: 1,
                    dynamic: false,
                    for_anonymous: false,
                    content: TaskContent::Empty,
                }],
            }],
        }];

        assert_eq!(
            actual.detect_changes(expected),
            vec![
                Modification::AddTechnology(Technology {
                    id: "technology_id".to_owned(),
                    name: "name".to_owned(),
                    description: Some("description".to_owned()),
                    position: 0,
                    image: Some("image".to_owned()),
                    sections_preview: vec![SectionPreview {
                        id: "section_id".to_owned(),
                        title: "title".to_owned(),
                    }],
                }),
                Modification::AddSection(Section {
                    id: "section_id".to_owned(),
                    technology_id: "technology_id".to_owned(),
                    title: "title".to_owned(),
                    position: 0,
                    description: Some("description".to_owned()),
                    image: Some("image".to_owned()),
                    tasks_preview: vec![TaskPreview {
                        id: "task_id".to_owned(),
                        title: "title".to_owned(),
                    }],
                }),
                Modification::AddTask(Task {
                    id: "task_id".to_owned(),
                    section_id: "section_id".to_owned(),
                    title: "title".to_owned(),
                    position: 0,
                    difficulty: 1,
                    dynamic: false,
                    for_anonymous: false,
                    content: TaskContent::Empty,
                })
            ]
        );
    }

    #[test]
    fn detect_changes_deletions() {
        let actual = RawTasksTree {
            technologies: vec![Technology {
                id: "technology_id".to_owned(),
                name: "name".to_owned(),
                description: Some("description".to_owned()),
                position: 0,
                image: Some("image".to_owned()),
                sections_preview: vec![SectionPreview {
                    id: "section_id".to_owned(),
                    title: "title".to_owned(),
                }],
            }],
            sections: vec![Section {
                id: "section_id".to_owned(),
                technology_id: "technology_id".to_owned(),
                title: "title".to_owned(),
                description: Some("description".to_owned()),
                position: 0,
                image: Some("image".to_owned()),
                tasks_preview: vec![TaskPreview {
                    id: "task_id".to_owned(),
                    title: "title".to_owned(),
                }],
            }],
            tasks: vec![Task {
                id: "task_id".to_owned(),
                section_id: "section_id".to_owned(),
                title: "title".to_owned(),
                position: 0,
                difficulty: 1,
                dynamic: false,
                for_anonymous: false,
                content: TaskContent::Empty,
            }],
        };

        let expected = vec![];

        assert_eq!(
            actual.detect_changes(expected),
            vec![
                Modification::RemoveTechnology(Technology {
                    id: "technology_id".to_owned(),
                    name: "name".to_owned(),
                    description: Some("description".to_owned()),
                    position: 0,
                    image: Some("image".to_owned()),
                    sections_preview: vec![SectionPreview {
                        id: "section_id".to_owned(),
                        title: "title".to_owned(),
                    }],
                }),
                Modification::RemoveSection(Section {
                    id: "section_id".to_owned(),
                    technology_id: "technology_id".to_owned(),
                    position: 0,
                    title: "title".to_owned(),
                    description: Some("description".to_owned()),
                    image: Some("image".to_owned()),
                    tasks_preview: vec![TaskPreview {
                        id: "task_id".to_owned(),
                        title: "title".to_owned(),
                    }],
                }),
                Modification::RemoveTask(Task {
                    id: "task_id".to_owned(),
                    section_id: "section_id".to_owned(),
                    title: "title".to_owned(),
                    position: 0,
                    difficulty: 1,
                    dynamic: false,
                    for_anonymous: false,
                    content: TaskContent::Empty,
                })
            ]
        );
    }

    #[test]
    fn detect_changes_updates() {
        let actual = RawTasksTree {
            technologies: vec![Technology {
                id: "technology_id".to_owned(),
                name: "name".to_owned(),
                description: Some("description".to_owned()),
                position: 0,
                image: Some("image".to_owned()),
                sections_preview: vec![SectionPreview {
                    id: "section_id".to_owned(),
                    title: "title".to_owned(),
                }],
            }],
            sections: vec![Section {
                id: "section_id".to_owned(),
                technology_id: "technology_id".to_owned(),
                title: "title".to_owned(),
                position: 0,
                description: Some("description".to_owned()),
                image: Some("image".to_owned()),
                tasks_preview: vec![TaskPreview {
                    id: "task_id".to_owned(),
                    title: "title".to_owned(),
                }],
            }],
            tasks: vec![Task {
                id: "task_id".to_owned(),
                section_id: "section_id".to_owned(),
                title: "title".to_owned(),
                position: 0,
                difficulty: 1,
                dynamic: false,
                for_anonymous: false,
                content: TaskContent::Empty,
            }],
        };

        let expected = vec![ExpectedTechnologyData {
            id: "technology_id".to_owned(),
            name: "name".to_owned(),
            description: Some("description".to_owned()),
            image: Some("image".to_owned()),
            sections: vec![ExpectedSectionData {
                id: "section_id".to_owned(),
                title: "title".to_owned(),
                description: Some("description".to_owned()),
                image: Some("image".to_owned()),
                tasks: vec![ExpectedTaskData {
                    id: "task_id".to_owned(),
                    title: "title".to_owned(),
                    difficulty: 2,
                    dynamic: true,
                    for_anonymous: true,
                    content: TaskContent::Empty,
                }],
            }],
        }];

        assert_eq!(
            actual.detect_changes(expected),
            vec![Modification::UpdateTask(Task {
                id: "task_id".to_owned(),
                section_id: "section_id".to_owned(),
                position: 0,
                title: "title".to_owned(),
                difficulty: 2,
                dynamic: true,
                for_anonymous: true,
                content: TaskContent::Empty,
            })]
        );
    }
}
