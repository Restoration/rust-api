use warp::Filter;
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

async fn list_users() -> Result<impl warp::Reply, warp::Rejection> {
    let users = vec![
        User { id: 1, name: "Alice".into() },
        User { id: 2, name: "Bob".into() },
    ];

    Ok(warp::reply::json(&users))
}

#[tokio::main]
async fn main() {
    let users_route = warp::path!("users")
        .and(warp::get())
        .and_then(list_users);

    warp::serve(users_route)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
