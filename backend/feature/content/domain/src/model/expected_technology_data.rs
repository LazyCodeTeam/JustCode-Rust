use super::{
    expected_section_data::ExpectedSectionData, full_content::FullContent, section::Section,
    section_preview::SectionPreview, task::Task, task_preview::TaskPreview, technology::Technology,
};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ExpectedTechnologyData {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub sections: Vec<ExpectedSectionData>,
}

impl From<Vec<ExpectedTechnologyData>> for FullContent {
    fn from(expected: Vec<ExpectedTechnologyData>) -> Self {
        let mut technologies = vec![];
        let mut sections = vec![];
        let mut tasks = vec![];

        for (index, technology) in expected.into_iter().enumerate() {
            let mut sections_preview = vec![];

            for (index, section) in technology.sections.into_iter().enumerate() {
                let mut tasks_preview = vec![];
                sections_preview.push(SectionPreview {
                    id: section.id.clone(),
                    title: section.title.clone(),
                });

                let mut current_task_position = 0;
                for task in section.tasks.into_iter() {
                    tasks_preview.push(TaskPreview {
                        id: task.id.clone(),
                        title: task.title.clone(),
                        for_anonymous: task.for_anonymous,
                    });
                    let position = if task.dynamic {
                        None
                    } else {
                        let result = Some(current_task_position);
                        current_task_position += 1;

                        result
                    };

                    tasks.push(Task {
                        id: task.id,
                        title: task.title,
                        difficulty: task.difficulty,
                        position,
                        for_anonymous: task.for_anonymous,
                        content: task.content,
                        section_id: section.id.clone(),
                    });
                }

                sections.push(Section {
                    id: section.id,
                    title: section.title,
                    description: section.description,
                    position: index as u64,
                    image: section.image,
                    technology_id: technology.id.clone(),
                    tasks_preview,
                });
            }

            technologies.push(Technology {
                id: technology.id,
                name: technology.name,
                description: technology.description,
                image: technology.image,
                position: index as u64,
                sections_preview,
            });
        }

        Self {
            technologies,
            sections,
            tasks,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{expected_task_data::ExpectedTaskData, task_content::TaskContent};

    use super::*;

    #[test]
    fn from_expected_technology_data() {
        let expected = vec![
            ExpectedTechnologyData {
                id: "id 1".to_string(),
                name: "name 1".to_string(),
                description: Some("description 1".to_string()),
                image: Some("image 1".to_string()),
                sections: vec![ExpectedSectionData {
                    id: "id 1".to_string(),
                    title: "title 1".to_string(),
                    description: Some("description 1".to_string()),
                    image: Some("image 1".to_string()),
                    tasks: vec![ExpectedTaskData {
                        id: "id 1".to_string(),
                        title: "title 1".to_string(),
                        difficulty: 1,
                        dynamic: true,
                        for_anonymous: true,
                        content: TaskContent::Empty,
                    }],
                }],
            },
            ExpectedTechnologyData {
                id: "id 2".to_string(),
                name: "name 2".to_string(),
                description: Some("description 2".to_string()),
                image: Some("image 2".to_string()),
                sections: vec![ExpectedSectionData {
                    id: "id 2".to_string(),
                    title: "title 2".to_string(),
                    description: Some("description 2".to_string()),
                    image: Some("image 2".to_string()),
                    tasks: vec![ExpectedTaskData {
                        id: "id 2".to_string(),
                        title: "title 2".to_string(),
                        difficulty: 2,
                        dynamic: false,
                        for_anonymous: false,
                        content: TaskContent::Empty,
                    }],
                }],
            },
        ];

        let raw_tasks_tree = FullContent::from(expected);

        assert_eq!(
            raw_tasks_tree,
            FullContent {
                technologies: vec![
                    Technology {
                        id: "id 1".to_string(),
                        name: "name 1".to_string(),
                        description: Some("description 1".to_string()),
                        position: 0,
                        image: Some("image 1".to_string()),
                        sections_preview: vec![SectionPreview {
                            id: "id 1".to_string(),
                            title: "title 1".to_string(),
                        }],
                    },
                    Technology {
                        id: "id 2".to_string(),
                        name: "name 2".to_string(),
                        position: 1,
                        description: Some("description 2".to_string()),
                        image: Some("image 2".to_string()),
                        sections_preview: vec![SectionPreview {
                            id: "id 2".to_string(),
                            title: "title 2".to_string(),
                        }],
                    },
                ],
                sections: vec![
                    Section {
                        id: "id 1".to_string(),
                        title: "title 1".to_string(),
                        description: Some("description 1".to_string()),
                        position: 0,
                        image: Some("image 1".to_string()),
                        technology_id: "id 1".to_string(),
                        tasks_preview: vec![TaskPreview {
                            id: "id 1".to_string(),
                            title: "title 1".to_string(),
                            for_anonymous: true,
                        }],
                    },
                    Section {
                        id: "id 2".to_string(),
                        title: "title 2".to_string(),
                        description: Some("description 2".to_string()),
                        position: 0,
                        image: Some("image 2".to_string()),
                        technology_id: "id 2".to_string(),
                        tasks_preview: vec![TaskPreview {
                            id: "id 2".to_string(),
                            title: "title 2".to_string(),
                            for_anonymous: false,
                        }],
                    },
                ],
                tasks: vec![
                    Task {
                        id: "id 1".to_string(),
                        title: "title 1".to_string(),
                        difficulty: 1,
                        position: None,
                        for_anonymous: true,
                        content: TaskContent::Empty,
                        section_id: "id 1".to_string(),
                    },
                    Task {
                        id: "id 2".to_string(),
                        title: "title 2".to_string(),
                        position: Some(0),
                        difficulty: 2,
                        for_anonymous: false,
                        content: TaskContent::Empty,
                        section_id: "id 2".to_string(),
                    },
                ],
            }
        );
    }
}
