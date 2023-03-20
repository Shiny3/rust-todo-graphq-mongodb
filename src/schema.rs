use std::sync::Arc;

use async_graphql::{Context, EmptySubscription, Object, Result, Schema, ID};
use futures_util::lock::Mutex;
use slab::Slab;

pub type ToDosSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
pub fn build_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Storage::default())
        .finish()
}

#[derive(Clone)]
pub struct ToDo {
    id: ID,
    description: String,
    completed: bool,
}

#[Object]
impl ToDo {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn description(&self) -> &str {
        &self.description
    }

    async fn completed(&self) -> bool {
        self.completed
    }
}

pub type Storage = Arc<Mutex<Slab<ToDo>>>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn todos(&self, ctx: &Context<'_>) -> Vec<ToDo> {
        let todos = ctx.data_unchecked::<Storage>().lock().await;
        todos.iter().map(|(_, book)| book).cloned().collect()
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_todo(&self, ctx: &Context<'_>, description: String, completed: bool) -> ID {
        let mut todos = ctx.data_unchecked::<Storage>().lock().await;
        let entry = todos.vacant_entry();
        let id: ID = entry.key().into();
        let todo = ToDo {
            id: id.clone(),
            description,
            completed,
        };
        entry.insert(todo);
        id
    }

    async fn delete_todo(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let mut todos = ctx.data_unchecked::<Storage>().lock().await;
        let id = id.parse::<usize>()?;
        if todos.contains(id) {
            todos.remove(id);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
