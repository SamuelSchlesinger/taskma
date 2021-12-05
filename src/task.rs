use crate::project::ProjectId;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskId(String);

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Status {
    category: String,
}

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Task {
    project: ProjectId,
    name: String,
    description: String,
    dependencies: BTreeMap<TaskId, Dependency>,
    assignee: Option<String>,
    status: Status,
}

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dependency {
    sort: String,
    justification: String,
}
