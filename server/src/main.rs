mod controllers;
use controllers::task;

#[async_std::main]
async fn main() -> tide::Result<()> {

    let mut app = tide::new();
    app.at("/task/:id").get(task::get);
    app.at("/task").get(task::get_all);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}






