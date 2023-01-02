use tide::{Body, Request, Response};


#[path = "../services/mod.rs"] mod services;

pub async fn get(req: Request<()>) -> tide::Result {
  let id = req.param("id")?;
  let mut res= Response::new(200);
          res.set_body(Body::from_json(&format!("{}{}", &"You called get method for id = ", id))?);
  Ok(res)
}

pub async fn get_all(req: Request<()>) -> tide::Result {
  let all_tasks = services::task::get_tasks();
  let mut res= Response::new(200);
          res.set_body(Body::from_json(&all_tasks)?);
  Ok(res)
}
