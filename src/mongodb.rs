use std::sync::Arc;
use futures_util::lock::Mutex;

use mongodb::{
		bson::{doc},// oid::ObjectId, 
		//results::{ InsertOneResult, UpdateResult, DeleteResult},
		 Client, //Collection,
		options::{ClientOptions, ResolverConfig},
    };
//use std::rc::Rc;use std::error::Error;
use crate::models::ToDo;
use crate::context::{MONGODB_URL};
use crate::schema::{StorageMongos};

/*
pub async fn mongodb()-> Result<(), Box< dyn Error>> {
let options =
      ClientOptions::parse_with_resolver_config(MONGODB_URL, ResolverConfig::cloudflare())
         .await?;

   let client = Client::with_options(options)?;

   println!("Databases:");
   for name in client.list_database_names(None, None).await? {
      println!("- {}", name);
   }

	//: mongodb::Collection<ToDo>
	let todos = client.database("rust").collection("todos");
	
	//todos.drop(None).await.unwrap();
    todos.insert_many(&[doc! {
   "id": 78,
   "description": "todo drop mango",
   "completed": true,
}, doc! {
   "id": 79,
   "description": "todo insert many mango",
   "completed": true,
},
doc! {
   "id": 79,
   "description": "todo delete mango",
   "completed": false,
},
doc! {
   "id": 77,
   "description": "todo update mango",
   "completed": false,
}], None)
        .await
        .unwrap();
	
	
	
	let  new_todo = doc! {
   "id": 77,
   "description": "todo update mango",
   "completed": false,
};
	let insert_result = todos.insert_one(new_todo.clone(), None).await?;
	println!("New todo ID: {}", insert_result.inserted_id);
	
	// Look up for a new challenge:
	/**/
	let new_todo7 = ToDo {
		 id: "777".into(),
 		 description: " update mongos".to_string(),
  		 completed: true,
	};
	//new::Collection<mongodb::bson::Document>
	 let mongo_storage:StorageMongos = Arc::new(Mutex::new(todos));

	//update_todo(mongo_storage, 777,new_todo7);//.await; 
	
	let todo_with_description_to_update = todos.find_one_and_update(
                doc! { "id": 77 },
                doc! { "description": "todo update moOongo :)".to_string() },
                None,
            )
            .await;
	
	println!("found undone todos: {:?} \n",  todo_with_description_to_update);
	
	// let obj_id = ObjectId::parse_str("641b703fe341ab62dca7ddd").unwrap();
	//let todo_with_id = todos
    //    .find_one(doc! { "_id": obj_id }, None)
    //    .await?;
    //println!("found todo with id {}: {:?} \n",  obj_id, todo_with_id); 


	let todo_undone = todos
        .find_one(doc! { "completed": false }, None)
        .await?;
    println!("found undone todos: {:?}",  todo_undone);




	let mut cursor = todos.find(None, None).await?;
	while let Some(todo) = &mut cursor.try_next().await? {
    println!("todo: {:?}", todo);
	
    //let todos_: Vec<ToDo> = Vec::new();	
}

	Ok(())
}
*/

/* 	let todos_undone = todos
        .find_all(doc! { "completed": false }, None)
        .await?;
    println!("\n found undone todos: {:?}",  todos_undone);

		
	let todo_with_id =  match todos.find_one(doc! { "_id": id }, None).await? {
            Some(todo_with_id) =>
			Ok(todo_with_id) ,
            _ => Err(Ok(())),//None => e),
        };

*/