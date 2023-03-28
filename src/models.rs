use async_graphql::{Object, ID}; //,ServerError,ServerResult,Positioned,ContextSelectionSet,OutputType};
                                 //use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
//use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToDo {
    pub id: ID,
    pub description: String,
    pub completed: bool,
}

#[Object]
impl ToDo {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn description(&self) -> &String {
        &self.description
    }

    async fn completed(&self) -> bool {
        self.completed
    }
}

pub trait Updater {
    fn set_description(&mut self, description: &str);
    fn set_completed(&mut self, completed: bool);

    fn update(&mut self, description: &str, completed: bool);
}
//async async  async
impl Updater for ToDo {
    fn set_description(&mut self, description: &str) {
        self.description = description.into();
    }
    fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }

    fn update(&mut self, description: &str, completed: bool) {
        self.set_description(description);
        self.set_completed(completed);
    }
}