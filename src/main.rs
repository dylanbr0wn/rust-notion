use indexmap::IndexMap;
use rocket::serde::json::Json;
mod notion;
mod utils;

#[macro_use]
extern crate rocket;

#[get("/page/<page_id>")]
async fn index(
    page_id: String,
    notion_client: &rocket::State<notion::NotionClient>,
) -> Option<Json<IndexMap<String, notion::types::Block>>> {
    let parsed_id = utils::parse_page_id(page_id);
    println!("{}", parsed_id);
    match notion_client.get_page(&parsed_id).await {
        Ok(page) => Some(Json(page.recordMap.block)),
        Err(_e) => None,
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(notion::NotionClient::new())
        .mount("/", routes![index])
}
