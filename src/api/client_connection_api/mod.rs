use actix_web_actors::ws;
use actix::{Actor, StreamHandler};
use actix_web::{web, HttpRequest, HttpResponse, Error};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.route("/", web::get().to(create_client_connection));
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

async fn create_client_connection(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
  let client_connection_manager = req.app_data::<Data<Addr<ClientConnectionManager>>().unwrap();
  let lobby_manager = req.app_data::<Data<Addr<LobbyManager>>().unwrap();
  client_connection_manager.send(
    ClientConnectionManagerMessage::CreateClientConnection { lobby_manager: lobby_manager })
    .wait().unwrap();
}
