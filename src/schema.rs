use async_graphql::{Context, EmptySubscription, Error, Object, Schema, ID};
use futures_util::lock::Mutex;
use std::sync::Arc; //

use slab::Slab;
/*
use mongodb::{
        bson::{ doc}, //oid::ObjectId,
        results::{ UpdateResult }, //InsertOneResult, , DeleteResult},
         Client, Collection,
        options::{ClientOptions, ResolverConfig},
    };
use futures_util::TryStreamExt;
use crate::context::{MONGODB_URL};Updater
*/
use crate::models::{ToDo, Updater};

pub type ToDosSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/*
pub async fn establish_mongodb()-> StorageMongos {
    let options =
            ClientOptions::parse_with_resolver_config(MONGODB_URL, ResolverConfig::cloudflare());

    let client = Client::with_options(options);

    println!("Databases:");
    for name in client.list_database_names(None, None) {
        println!("- {}", name);
    }
    let todos = client.database("rust").collection("todos");
    let mongo_storage:StorageMongos = Arc::new(Mutex::new(todos));
    mongo_storage
}*/

pub fn build_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Storage::default())
        //.data(establish_mongodb())
        .finish()
}

//pub type StorageMongos = Arc<Mutex<Collection<mongodb::bson::Document>>>;
pub type Storage = Arc<Mutex<Slab<ToDo>>>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn todos(&self, ctx: &Context<'_>) -> Vec<ToDo> {
        let todos = ctx.data_unchecked::<Storage>().lock().await;
        for todo1 in todos.iter() {
            println!("{:?}", todo1);
        }
        todos.iter().map(|(_, todo)| todo).cloned().collect()
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
    /*
    pub async fn update_todo(&self, ctx: &Context<'_>, id: i64, new_todo: ToDo) -> Result<ToDo, Error>//
    {
        //let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"id": id};
        let new_todo = doc! {
            "$set":
                {
                    "id":  new_todo.id,
                    "description": new_todo.description,
                    "completed": new_todo.completed,
                },
        };

        let zero_todo = doc! {
            "$set":
                {
                    "id":  new_todo.id,
                    "description": new_todo.description,
                    "completed": new_todo.completed,
                },
        };

        let mut todos = ctx.data_unchecked::<StorageMongos>().lock().await;
        let updated_todo = todos
            .update_one(filter, new_todo, None)
            .await;
        //	.ok()
        //	.expect("Error updating user");

         match updated_todo {
                Ok(update) => {
                    if update.matched_count == 1 {
                        let updated_todo_info = self.get_todo(&id).await;
                        return match updated_todo_info {
                            Ok(todo) => Ok(updated_todo_info),
                            Err(err) => Err(err),
                        };
                    } else {
                        Err(())
                    }
                }
            }

        //Ok(updated_todo)
    }


    pub async fn get_todo(&self, ctx: &Context<'_>,  id: i64 ) -> Result<ToDo, Error> {
        let mut todos = ctx.data_unchecked::<StorageMongos>().lock().await;
            //let obj_id = ObjectId::parse_str(id).unwrap();&Stringobj_id
            let filter = doc! {"id": id};
            let todo = todos.find_one(filter, None)
                .await;
            //	.ok()
            //	.expect("Error getting todo's detail");
            Ok(todo.unwrap())
        }
    */
    /*UPDATE MJUTATION */
    async fn update_todo(
        &self,
        ctx: &Context<'_>,
        id: ID,
        description: String,
        completed: bool,
    ) -> Result<bool, Error> {
        let mut todos = ctx.data_unchecked::<Storage>().lock().await;
        let id = id.parse::<usize>()?;
        let exist = todos.contains(id);

        if exist {
            let todo: &mut ToDo = todos.get_mut(id).unwrap();
            todo.set_description(&description);
            todo.set_completed(completed);
            //todo.update(&description, completed);
            return Ok(bool);
        };
        Ok(false)
    }

    async fn delete_todo(&self, ctx: &Context<'_>, id: ID) -> Result<bool, Error> {
        let mut todos = ctx.data_unchecked::<Storage>().lock().await;
        let id = id.parse::<usize>()?;
        if todos.contains(id) {
            todos.remove(id);
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
}
