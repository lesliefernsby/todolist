use server::models::*;
use diesel::prelude::*;


pub fn get_tasks() -> String{
  use server::schema::tasks::dsl::*;

  let connection = &mut server::establish_connection();
  let results = tasks
      .limit(5)
      .load::<Task>(connection)
      .expect("Error loading tasks");


  let json = serde_json::to_string(&results).expect("error");
  return json;
}
