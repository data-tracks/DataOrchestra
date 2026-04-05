use crate::object::Object;

pub async fn connect(_store: &Object) {
    /*
     let connection_string = postgres.get_connection_string();
     let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await.unwrap();

     tokio::spawn(async move {
         if let Err(e) = connection.await {
             eprintln!("connection error: {}", e);
         }
     });
    */
}
