use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use webhook::client::{WebhookClient, WebhookResult};
use serde::Deserialize;

#[derive(Deserialize)]
struct ReqInfo {
    message: String
}

const AVATAR_URI: &'static str = "https://cdn.discordapp.com/avatars/312157715449249795/a_b8b3b0c35f3dee2b6586a0dd58697e29.png";

async fn fire(content: &String) -> WebhookResult<()> {
    let client = WebhookClient::new("https://discord.com/api/webhooks/1271277983327322236/l_oEjUGJbXko3uEh346pYfBs787_yUzlMDom7xzr8ZHZh-f-vKWr4aR5HcNc9M1rhsfC");
    let webhook_info = client.get_information().await?;
    println!("webhook: {:?}", webhook_info);
    print!("{}", content);

    client.send(|message| message
        .content("@everyone")
        .username("crabuzz")
        .avatar_url(AVATAR_URI)
        .embed(|embed| embed
            .title("Webhook")
            .description(content)
            .footer("Footer", Some(String::from(AVATAR_URI)))
            .image(AVATAR_URI)
            .thumbnail(AVATAR_URI)
            .author("CRABUS!🦀", Some(String::from(AVATAR_URI)), Some(String::from(AVATAR_URI)))
            .field("name", "value", false))).await?;

    Ok(())
}


#[get("/pop/{message}")]
async fn endpoint(info: web::Path<ReqInfo>) -> impl Responder {
    let _ = fire(&info.message).await;
    HttpResponse::Ok().body(info.message.to_owned())

    // let url = dotenv::var("DISCORD_URI")?;
    //let client = WebhookClient::new(&url);

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenv::dotenv()?; 
    HttpServer::new(|| {
        App::new()
            .service(endpoint)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}