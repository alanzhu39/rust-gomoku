use actix_web_actors::ws;
use actix::{Actor, StreamHandler};
use actix_web::{web, HttpRequest, HttpResponse, Error};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.route("/create", web::get().to(create_lobby));
}

struct MyWs;

impl Actor for MyWs {
  type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
      Ok(ws::Message::Text(text)) => ctx.text(text),
      Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
      _ => (),
    }
  }
}

async fn create_lobby(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
  let resp = ws::start(MyWs {}, &req, stream);
  println!("{:?}", resp);
  resp
}
