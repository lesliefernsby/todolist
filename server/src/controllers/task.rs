use tide::{Body, Request, Response};

pub async fn get(req: Request<()>) -> tide::Result {
  let id = req.param("id")?;
  let mut res= Response::new(200);
          res.set_body(Body::from_json(&format!("{}{}", &"You called get method for id = ", id))?);
  Ok(res)
}
