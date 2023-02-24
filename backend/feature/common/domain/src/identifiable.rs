use std::hash::Hash;

pub trait Identifiable {
    type Id: Hash + Eq + Clone;

    fn id(&self) -> &Self::Id;
}
