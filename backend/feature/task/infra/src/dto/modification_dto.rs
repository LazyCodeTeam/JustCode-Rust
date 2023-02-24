use super::{section_dto::SectionDto, task_dto::TaskDto, technology_dto::TechnologyDto};
use aws_sdk_dynamodb::model::{AttributeValue, PutRequest, WriteRequest};
use common_infra::dynamodb_identifiable::DynamoDbIdentifiable;
use serde::{Deserialize, Serialize};
use serde_dynamo::to_item;
use task_domain::model::modification::Modification;

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

impl From<Modification> for ModificationDto {
    fn from(value: Modification) -> Self {
        match value {
            Modification::AddTechnology(technology) => {
                ModificationDto::AddTechnology(technology.into())
            }
            Modification::AddSection(section) => ModificationDto::AddSection(section.into()),
            Modification::AddTask(task) => ModificationDto::AddTask(task.into()),
            Modification::RemoveTechnology(technology) => {
                ModificationDto::RemoveTechnology(technology.into())
            }
            Modification::RemoveSection(section) => ModificationDto::RemoveSection(section.into()),
            Modification::RemoveTask(task) => ModificationDto::RemoveTask(task.into()),
            Modification::UpdateTechnology(technology) => {
                ModificationDto::UpdateTechnology(technology.into())
            }
            Modification::UpdateSection(section) => ModificationDto::UpdateSection(section.into()),
            Modification::UpdateTask(task) => ModificationDto::UpdateTask(task.into()),
        }
    }
}

impl From<ModificationDto> for Modification {
    fn from(value: ModificationDto) -> Self {
        match value {
            ModificationDto::AddTechnology(technology) => {
                Modification::AddTechnology(technology.into())
            }
            ModificationDto::AddSection(section) => Modification::AddSection(section.into()),
            ModificationDto::AddTask(task) => Modification::AddTask(task.into()),
            ModificationDto::RemoveTechnology(technology) => {
                Modification::RemoveTechnology(technology.into())
            }
            ModificationDto::RemoveSection(section) => Modification::RemoveSection(section.into()),
            ModificationDto::RemoveTask(task) => Modification::RemoveTask(task.into()),
            ModificationDto::UpdateTechnology(technology) => {
                Modification::UpdateTechnology(technology.into())
            }
            ModificationDto::UpdateSection(section) => Modification::UpdateSection(section.into()),
            ModificationDto::UpdateTask(task) => Modification::UpdateTask(task.into()),
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
            aws_sdk_dynamodb::model::DeleteRequest::builder()
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
    use task_domain::model::{section::Section, task::Task, technology::Technology};

    use super::*;

    #[test]
    fn from_modification() {
        let technology = Technology::default();
        let section = Section::default();
        let task = Task::default();

        assert_eq!(
            ModificationDto::from(Modification::AddTechnology(technology.clone())),
            ModificationDto::AddTechnology(technology.clone().into())
        );
        assert_eq!(
            ModificationDto::from(Modification::AddSection(section.clone())),
            ModificationDto::AddSection(section.clone().into())
        );
        assert_eq!(
            ModificationDto::from(Modification::AddTask(task.clone())),
            ModificationDto::AddTask(task.clone().into())
        );
        assert_eq!(
            ModificationDto::from(Modification::RemoveTechnology(technology.clone())),
            ModificationDto::RemoveTechnology(technology.clone().into())
        );
        assert_eq!(
            ModificationDto::from(Modification::RemoveSection(section.clone())),
            ModificationDto::RemoveSection(section.clone().into())
        );
        assert_eq!(
            ModificationDto::from(Modification::RemoveTask(task.clone())),
            ModificationDto::RemoveTask(task.clone().into())
        );
        assert_eq!(
            ModificationDto::from(Modification::UpdateTechnology(technology.clone())),
            ModificationDto::UpdateTechnology(technology.into())
        );
        assert_eq!(
            ModificationDto::from(Modification::UpdateSection(section.clone())),
            ModificationDto::UpdateSection(section.into())
        );
        assert_eq!(
            ModificationDto::from(Modification::UpdateTask(task.clone())),
            ModificationDto::UpdateTask(task.into())
        );
    }

    #[test]
    fn from_modification_dto() {
        let technology = TechnologyDto::default();
        let section = SectionDto::default();
        let task = TaskDto::default();

        assert_eq!(
            Modification::from(ModificationDto::AddTechnology(technology.clone())),
            Modification::AddTechnology(technology.clone().into())
        );
        assert_eq!(
            Modification::from(ModificationDto::AddSection(section.clone())),
            Modification::AddSection(section.clone().into())
        );
        assert_eq!(
            Modification::from(ModificationDto::AddTask(task.clone())),
            Modification::AddTask(task.clone().into())
        );
        assert_eq!(
            Modification::from(ModificationDto::RemoveTechnology(technology.clone())),
            Modification::RemoveTechnology(technology.clone().into())
        );
        assert_eq!(
            Modification::from(ModificationDto::RemoveSection(section.clone())),
            Modification::RemoveSection(section.clone().into())
        );
        assert_eq!(
            Modification::from(ModificationDto::RemoveTask(task.clone())),
            Modification::RemoveTask(task.clone().into())
        );
        assert_eq!(
            Modification::from(ModificationDto::UpdateTechnology(technology.clone())),
            Modification::UpdateTechnology(technology.into())
        );
        assert_eq!(
            Modification::from(ModificationDto::UpdateSection(section.clone())),
            Modification::UpdateSection(section.into())
        );
        assert_eq!(
            Modification::from(ModificationDto::UpdateTask(task.clone())),
            Modification::UpdateTask(task.into())
        );
    }
}
