use crate::{person, project, task};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord)]
enum Statement {}

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct ProjectModel {
    tickets: BTreeMap<task::TaskId, task::Task>,
    relationships: BTreeMap<task::TaskId, BTreeMap<String, BTreeSet<task::TaskId>>>,
    tags: BTreeMap<task::TaskId, String>,
}

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct System {
    commands: Vec<Statement>,
    people: BTreeMap<person::PersonId, person::Person>,
    model: BTreeMap<project::ProjectId, ProjectModel>,
}

impl System {
    fn execute(&mut self, statement: Statement) {
        match statement {}
    }
}
