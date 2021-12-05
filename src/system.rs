use crate::{person, project, task};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord)]
enum Command {
    NewPerson(person::PersonId, person::Person),
    NewProject(project::ProjectId, project::Project),
    NewTask(project::ProjectId, task::TaskId, task::Task),
}

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct ProjectModel {
    name: String,
    description: String,
    members: BTreeSet<person::PersonId>,
    tasks: BTreeMap<task::TaskId, task::Task>,
    relationships: BTreeMap<task::TaskId, BTreeMap<String, BTreeSet<task::TaskId>>>,
    tags: BTreeMap<task::TaskId, String>,
}

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct Model {
    people: BTreeMap<person::PersonId, person::Person>,
    projects: BTreeMap<project::ProjectId, ProjectModel>,
}

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct System {
    commands: Vec<Command>,
    model: Model,
}

impl Model {
    fn execute(&mut self, command: &Command) -> bool {
        use std::collections::btree_map::Entry;
        use Command::*;
        match command {
            NewPerson(person_id, person) => match self.people.entry(person_id.clone()) {
                // Person doesn't exist. Just create them!
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(person.clone());
                    true
                }
                // Person already exists, so we disallow.
                Entry::Occupied(_occupied_entry) => false,
            },
            NewProject(project_id, project) => match self.projects.entry(project_id.clone()) {
                // Project doesn't exist, so we make sure all of its projected
                // members do.
                Entry::Vacant(vacant_entry) => {
                    let mut all_members_exist = true;
                    for member in project.members.iter() {
                        all_members_exist = all_members_exist && self.people.contains_key(&member);
                    }
                    if all_members_exist {
                        vacant_entry.insert(ProjectModel {
                            name: project.name.clone(),
                            description: project.description.clone(),
                            members: project.members.clone(),
                            tasks: BTreeMap::new(),
                            relationships: BTreeMap::new(),
                            tags: BTreeMap::new(),
                        });
                        true
                    } else {
                        false
                    }
                }
                Entry::Occupied(_occupied_entry) => false,
            },
        }
    }
}
