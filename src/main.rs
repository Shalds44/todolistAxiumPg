use axum::{
    response::Html, routing::get, Json, Router
};
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(handler_home))
        .route("/about", get(handler_about))
        .route("/json", get(handler_json));

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

async fn handler_json() -> Json<String>{
    let conn_string = "host=localhost user=postgres password=password port=5432 dbname=student connect_timeout=10";

    let (client, connection) =
        tokio_postgres::connect(conn_string, NoTls).await.unwrap();

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