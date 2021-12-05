use crate::person::PersonId;
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ProjectId(String);

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub members: BTreeSet<PersonId>,
}
