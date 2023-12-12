use actix::{Actor, StreamHandler};
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
    use actix_web_actors::ws;

struct ChatServer;

impl Actor for ChatServer {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatServer {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn chat_route(req: HttpRequest, stream: web::Payload) -> impl Responder {
    ws::start(ChatServer, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/chat", web::get().to(chat_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
