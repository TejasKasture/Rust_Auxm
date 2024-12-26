use hello_world::run;

#[tokio::main]
async fn main() {
  run().await;
}


// async fn hello_world()->String
// {
//     return "Hello Tejas using watch".to_owned()
// }