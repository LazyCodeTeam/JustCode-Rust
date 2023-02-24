use crate::model::modification::Modification;

pub trait IntoModification {
    fn into_add_modification(self) -> Modification;

    fn into_remove_modification(self) -> Modification;

    fn into_update_modification(self) -> Modification;
}
