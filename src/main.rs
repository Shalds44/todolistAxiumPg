use axum::{
    extract::Path, http::StatusCode, response::Html, routing::{delete, get, post}, Json, Router
};
use tokio_postgres::{NoTls, Error};
use todolistAxiumPg::{CONN_STRING};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(handler_home))
        .route("/about", get(handler_about))
        .route("/add", post(handler_add))
        .route("/remove/:id", delete(handler_remove))
        .route("/task/:id", get(handler_get_one));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn handler_home() -> Html<&'static str>{
    
    Html("<h5>Hello, World!</h5>")
}

async fn handler_about() -> Html<&'static str>{
    Html("<h5>about</h5>")
}

async fn handler_get_one(Path(id): Path<i64>) -> Json<String>{

    let (client, connection) =
        tokio_postgres::connect(CONN_STRING, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client
    .query("SELECT lastname FROM student LIMIT 1", &[])
    .await.unwrap();

    let mut result = String::new();
    // Traiter les résultats
    for row in rows {
        let lastname: &str = row.get(0);
        result = lastname.to_string();
    }

    Json(result)
}

async fn handler_add() -> Json<String>{
    let (client, connection) =
        tokio_postgres::connect(CONN_STRING, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client
    .query(
        "INSERT INTO task (title, content, state) VALUES ($1, $2, $3)", 
        &[&"manger haricot", &"aller à superu", &true]
    )
    .await.unwrap();

    let mut result = String::new();
    // Traiter les résultats
    for row in rows {
        let lastname: &str = row.get(0);
        result = lastname.to_string();
    }

    Json(result)
}

async fn handler_remove(Path(id): Path<i64>) -> StatusCode{

    let (client, connection) =
    tokio_postgres::connect(CONN_STRING, NoTls).await.unwrap();
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let row_affected = client
    .execute(
        "DELETE FROM task WHERE id = $1", 
        &[&id]
    )
    .await.unwrap();

    if row_affected == 1 {
        StatusCode::OK
    }else{
        StatusCode::NOT_FOUND
    }

}