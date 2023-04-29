use crate::{MapFrom, MapInto};

use super::{section_dto::SectionDto, task_dto::TaskDto, technology_dto::TechnologyDto};
use aws_sdk_dynamodb::types::{AttributeValue, PutRequest, WriteRequest};
use common_domain::error::ResultLogExt;
use common_infra::dynamodb::identifiable::DynamoDbIdentifiable;
use content_domain::model::modification::Modification;
use serde::{Deserialize, Serialize};
use serde_dynamo::to_item;
use snafu::ResultExt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ModificationDto {
    AddTechnology(TechnologyDto),
    AddSection(SectionDto),
    AddTask(TaskDto),
    RemoveTechnology(TechnologyDto),
    RemoveSection(SectionDto),
    RemoveTask(TaskDto),
    UpdateTechnology(TechnologyDto),
    UpdateSection(SectionDto),
    UpdateTask(TaskDto),
}

impl MapFrom<Modification> for ModificationDto {
    fn map_from(model: Modification) -> Self {
        match model {
            Modification::AddTechnology(technology) => {
                ModificationDto::AddTechnology(technology.map_into())
            }
            Modification::AddSection(section) => ModificationDto::AddSection(section.map_into()),
            Modification::AddTask(task) => ModificationDto::AddTask(task.map_into()),
            Modification::RemoveTechnology(technology) => {
                ModificationDto::RemoveTechnology(technology.map_into())
            }
            Modification::RemoveSection(section) => {
                ModificationDto::RemoveSection(section.map_into())
            }
            Modification::RemoveTask(task) => ModificationDto::RemoveTask(task.map_into()),
            Modification::UpdateTechnology(technology) => {
                ModificationDto::UpdateTechnology(technology.map_into())
            }
            Modification::UpdateSection(section) => {
                ModificationDto::UpdateSection(section.map_into())
            }
            Modification::UpdateTask(task) => ModificationDto::UpdateTask(task.map_into()),
        }
    }
}

impl MapFrom<ModificationDto> for Modification {
    fn map_from(dto: ModificationDto) -> Self {
        match dto {
            ModificationDto::AddTechnology(technology) => {
                Modification::AddTechnology(technology.map_into())
            }
            ModificationDto::AddSection(section) => Modification::AddSection(section.map_into()),
            ModificationDto::AddTask(task) => Modification::AddTask(task.map_into()),
            ModificationDto::RemoveTechnology(technology) => {
                Modification::RemoveTechnology(technology.map_into())
            }
            ModificationDto::RemoveSection(section) => {
                Modification::RemoveSection(section.map_into())
            }
            ModificationDto::RemoveTask(task) => Modification::RemoveTask(task.map_into()),
            ModificationDto::UpdateTechnology(technology) => {
                Modification::UpdateTechnology(technology.map_into())
            }
            ModificationDto::UpdateSection(section) => {
                Modification::UpdateSection(section.map_into())
            }
            ModificationDto::UpdateTask(task) => Modification::UpdateTask(task.map_into()),
        }
    }
}

impl TryFrom<ModificationDto> for WriteRequest {
    type Error = common_domain::error::Error;

    fn try_from(value: ModificationDto) -> Result<Self, Self::Error> {
        match value {
            ModificationDto::AddTechnology(technology) => get_put_write_request(technology),
            ModificationDto::AddSection(section) => get_put_write_request(section),
            ModificationDto::AddTask(task) => get_put_write_request(task),
            ModificationDto::UpdateTechnology(technology) => get_put_write_request(technology),
            ModificationDto::UpdateSection(section) => get_put_write_request(section),
            ModificationDto::UpdateTask(task) => get_put_write_request(task),
            ModificationDto::RemoveTechnology(technology) => {
                Ok(get_delete_write_request(technology))
            }
            ModificationDto::RemoveSection(section) => Ok(get_delete_write_request(section)),
            ModificationDto::RemoveTask(task) => Ok(get_delete_write_request(task)),
        }
    }
}

fn get_delete_write_request(item: impl DynamoDbIdentifiable) -> WriteRequest {
    WriteRequest::builder()
        .delete_request(
            aws_sdk_dynamodb::types::DeleteRequest::builder()
                .key("PK", AttributeValue::S(item.pk()))
                .key("SK", AttributeValue::S(item.sk()))
                .build(),
        )
        .build()
}

fn get_put_write_request(
    item: impl Serialize,
) -> Result<WriteRequest, common_domain::error::Error> {
    Ok(WriteRequest::builder()
        .put_request(
            PutRequest::builder()
                .set_item(Some(
                    to_item(item)
                        .whatever_context("Failed parse modification item")
                        .with_error_log()?,
                ))
                .build(),
        )
        .build())
}

#[cfg(test)]
mod tests {
    use content_domain::model::{section::Section, task::Task, technology::Technology};

    use super::*;

    #[test]
    fn from_modification() {
        let technology = Technology::default();
        let section = Section::default();
        let task = Task::default();

        assert_eq!(
            ModificationDto::map_from(Modification::AddTechnology(technology.clone())),
            ModificationDto::AddTechnology(technology.clone().map_into())
        );
        assert_eq!(
            ModificationDto::map_from(Modification::AddSection(section.clone())),
            ModificationDto::AddSection(section.clone().map_into())
        );
        assert_eq!(
            ModificationDto::map_from(Modification::AddTask(task.clone())),
            ModificationDto::AddTask(task.clone().map_into())
        );
        assert_eq!(
            ModificationDto::map_from(Modification::RemoveTechnology(technology.clone())),
            ModificationDto::RemoveTechnology(technology.clone().map_into())
        );
        assert_eq!(
            ModificationDto::map_from(Modification::RemoveSection(section.clone())),
            ModificationDto::RemoveSection(section.clone().map_into())
        );
        assert_eq!(
            ModificationDto::map_from(Modification::RemoveTask(task.clone())),
            ModificationDto::RemoveTask(task.clone().map_into())
        );
        assert_eq!(
            ModificationDto::map_from(Modification::UpdateTechnology(technology.clone())),
            ModificationDto::UpdateTechnology(technology.map_into())
        );
        assert_eq!(
            ModificationDto::map_from(Modification::UpdateSection(section.clone())),
            ModificationDto::UpdateSection(section.map_into())
        );
        assert_eq!(
            ModificationDto::map_from(Modification::UpdateTask(task.clone())),
            ModificationDto::UpdateTask(task.map_into())
        );
    }

    #[test]
    fn from_modification_dto() {
        let technology = TechnologyDto::default();
        let section = SectionDto::default();
        let task = TaskDto::default();

        assert_eq!(
            Modification::map_from(ModificationDto::AddTechnology(technology.clone())),
            Modification::AddTechnology(technology.clone().map_into())
        );
        assert_eq!(
            Modification::map_from(ModificationDto::AddSection(section.clone())),
            Modification::AddSection(section.clone().map_into())
        );
        assert_eq!(
            Modification::map_from(ModificationDto::AddTask(task.clone())),
            Modification::AddTask(task.clone().map_into())
        );
        assert_eq!(
            Modification::map_from(ModificationDto::RemoveTechnology(technology.clone())),
            Modification::RemoveTechnology(technology.clone().map_into())
        );
        assert_eq!(
            Modification::map_from(ModificationDto::RemoveSection(section.clone())),
            Modification::RemoveSection(section.clone().map_into())
        );
        assert_eq!(
            Modification::map_from(ModificationDto::RemoveTask(task.clone())),
            Modification::RemoveTask(task.clone().map_into())
        );
        assert_eq!(
            Modification::map_from(ModificationDto::UpdateTechnology(technology.clone())),
            Modification::UpdateTechnology(technology.map_into())
        );
        assert_eq!(
            Modification::map_from(ModificationDto::UpdateSection(section.clone())),
            Modification::UpdateSection(section.map_into())
        );
        assert_eq!(
            Modification::map_from(ModificationDto::UpdateTask(task.clone())),
            Modification::UpdateTask(task.map_into())
        );
    }
}
