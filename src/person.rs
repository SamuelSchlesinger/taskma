use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PersonId(String);

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Person {
    name: String,
    email: String,
    bio: String,
}
