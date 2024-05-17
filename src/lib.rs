use std::sync::{Arc, Mutex};

use tokio_postgres::{tls::NoTlsStream, Client, Connection, Error, NoTls, Socket};

pub const CONN_STRING:&str = "host=localhost user=postgres password=password port=5432 dbname=student connect_timeout=10";

pub type SharedClient = Arc<Mutex<Client>>;

pub async fn connect_to_db() -> Result<(SharedClient, Connection<Socket, NoTlsStream>), Error> {
    let (client, connection) = tokio_postgres::connect(CONN_STRING, NoTls).await?;
    let shared_client = Arc::new(Mutex::new(client));
    Ok((shared_client, connection))
}

