mod lobby_manager;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.route("", web::get().to(search_user))
    .route("", web::post().to(register))
    .service(
        web::scope("/me")
            .route("", web::get().to(me))
            .service(web::scope("/friends").configure(friends::config)),
    )
    .route("/register", web::post().to(register))
    .route("/login", web::post().to(login))
    .route("/logout", web::post().to(logout))
    .route("/{user_id}", web::get().to(get_user));
}
