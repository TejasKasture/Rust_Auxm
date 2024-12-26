mod routes;
use routes::create_routes;
//use axum::{body::Body, routing::get, Router};


pub async fn run()
{
     // build our application with a single route
  //  let app = create_routes();
 
   let app=create_routes(); //Router::new().route("/hello", get(||async {"Hello Tejas Kasture"}));

     // run our app with hyper, listening globally on port 3000
     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
     axum::serve(listener, app).await.unwrap();


}