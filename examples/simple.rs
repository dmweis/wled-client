use wled_client;

#[tokio::main]
async fn main() {
    println!("Hello example");
    let client = wled_client::Wled::new("http://192.168.0.111").unwrap();
    println!("Info {:?}\n", client.get_info().await.unwrap());
    println!("Effects {:?}", client.get_effects().await.unwrap());
}
