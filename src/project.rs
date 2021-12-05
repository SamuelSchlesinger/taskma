use crate::person::PersonId;
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProjectId(String);

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Project {
    name: String,
    description: String,
    members: BTreeSet<PersonId>,
}
