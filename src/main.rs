use felidae_network_client::id_types::{whitelist_id_type, IdType};

#[tokio::main]
async fn main() {
    let id_type = IdType::id_indian_passport();
    if let Err(e) = whitelist_id_type(id_type).await {
        println!("error is : {e:?}");
    };
}
