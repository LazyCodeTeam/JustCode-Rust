use crate::{FromDto, FromModel, IntoDto, IntoModel};

use super::{section_dto::SectionDto, task_dto::TaskDto, technology_dto::TechnologyDto};
use aws_sdk_dynamodb::types::{AttributeValue, PutRequest, WriteRequest};
use common_infra::dynamodb_identifiable::DynamoDbIdentifiable;
use content_domain::model::modification::Modification;
use serde::{Deserialize, Serialize};
use serde_dynamo::to_item;

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

impl FromModel<Modification> for ModificationDto {
    fn from_model(model: Modification) -> Self {
        match model {
            Modification::AddTechnology(technology) => {
                ModificationDto::AddTechnology(technology.into_dto())
            }
            Modification::AddSection(section) => ModificationDto::AddSection(section.into_dto()),
            Modification::AddTask(task) => ModificationDto::AddTask(task.into_dto()),
            Modification::RemoveTechnology(technology) => {
                ModificationDto::RemoveTechnology(technology.into_dto())
            }
            Modification::RemoveSection(section) => {
                ModificationDto::RemoveSection(section.into_dto())
            }
            Modification::RemoveTask(task) => ModificationDto::RemoveTask(task.into_dto()),
            Modification::UpdateTechnology(technology) => {
                ModificationDto::UpdateTechnology(technology.into_dto())
            }
            Modification::UpdateSection(section) => {
                ModificationDto::UpdateSection(section.into_dto())
            }
            Modification::UpdateTask(task) => ModificationDto::UpdateTask(task.into_dto()),
        }
    }
}

impl FromDto<ModificationDto> for Modification {
    fn from_dto(dto: ModificationDto) -> Self {
        match dto {
            ModificationDto::AddTechnology(technology) => {
                Modification::AddTechnology(technology.into_model())
            }
            ModificationDto::AddSection(section) => Modification::AddSection(section.into_model()),
            ModificationDto::AddTask(task) => Modification::AddTask(task.into_model()),
            ModificationDto::RemoveTechnology(technology) => {
                Modification::RemoveTechnology(technology.into_model())
            }
            ModificationDto::RemoveSection(section) => {
                Modification::RemoveSection(section.into_model())
            }
            ModificationDto::RemoveTask(task) => Modification::RemoveTask(task.into_model()),
            ModificationDto::UpdateTechnology(technology) => {
                Modification::UpdateTechnology(technology.into_model())
            }
            ModificationDto::UpdateSection(section) => {
                Modification::UpdateSection(section.into_model())
            }
            ModificationDto::UpdateTask(task) => Modification::UpdateTask(task.into_model()),
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
                .set_item(Some(to_item(item).map_err(parsing_failed_error)?))
                .build(),
        )
        .build())
}

fn parsing_failed_error(serde_dynamo_error: serde_dynamo::Error) -> common_domain::error::Error {
    common_domain::error::Error::unknown(format!(
        "Failed to parse modification: {serde_dynamo_error:?}",
    ))
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
            ModificationDto::from_model(Modification::AddTechnology(technology.clone())),
            ModificationDto::AddTechnology(technology.clone().into_dto())
        );
        assert_eq!(
            ModificationDto::from_model(Modification::AddSection(section.clone())),
            ModificationDto::AddSection(section.clone().into_dto())
        );
        assert_eq!(
            ModificationDto::from_model(Modification::AddTask(task.clone())),
            ModificationDto::AddTask(task.clone().into_dto())
        );
        assert_eq!(
            ModificationDto::from_model(Modification::RemoveTechnology(technology.clone())),
            ModificationDto::RemoveTechnology(technology.clone().into_dto())
        );
        assert_eq!(
            ModificationDto::from_model(Modification::RemoveSection(section.clone())),
            ModificationDto::RemoveSection(section.clone().into_dto())
        );
        assert_eq!(
            ModificationDto::from_model(Modification::RemoveTask(task.clone())),
            ModificationDto::RemoveTask(task.clone().into_dto())
        );
        assert_eq!(
            ModificationDto::from_model(Modification::UpdateTechnology(technology.clone())),
            ModificationDto::UpdateTechnology(technology.into_dto())
        );
        assert_eq!(
            ModificationDto::from_model(Modification::UpdateSection(section.clone())),
            ModificationDto::UpdateSection(section.into_dto())
        );
        assert_eq!(
            ModificationDto::from_model(Modification::UpdateTask(task.clone())),
            ModificationDto::UpdateTask(task.into_dto())
        );
    }

    #[test]
    fn from_modification_dto() {
        let technology = TechnologyDto::default();
        let section = SectionDto::default();
        let task = TaskDto::default();

        assert_eq!(
            Modification::from_dto(ModificationDto::AddTechnology(technology.clone())),
            Modification::AddTechnology(technology.clone().into_model())
        );
        assert_eq!(
            Modification::from_dto(ModificationDto::AddSection(section.clone())),
            Modification::AddSection(section.clone().into_model())
        );
        assert_eq!(
            Modification::from_dto(ModificationDto::AddTask(task.clone())),
            Modification::AddTask(task.clone().into_model())
        );
        assert_eq!(
            Modification::from_dto(ModificationDto::RemoveTechnology(technology.clone())),
            Modification::RemoveTechnology(technology.clone().into_model())
        );
        assert_eq!(
            Modification::from_dto(ModificationDto::RemoveSection(section.clone())),
            Modification::RemoveSection(section.clone().into_model())
        );
        assert_eq!(
            Modification::from_dto(ModificationDto::RemoveTask(task.clone())),
            Modification::RemoveTask(task.clone().into_model())
        );
        assert_eq!(
            Modification::from_dto(ModificationDto::UpdateTechnology(technology.clone())),
            Modification::UpdateTechnology(technology.into_model())
        );
        assert_eq!(
            Modification::from_dto(ModificationDto::UpdateSection(section.clone())),
            Modification::UpdateSection(section.into_model())
        );
        assert_eq!(
            Modification::from_dto(ModificationDto::UpdateTask(task.clone())),
            Modification::UpdateTask(task.into_model())
        );
    }
}
