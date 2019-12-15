#[macro_use]
extern crate serde_derive;

use actix_files as fs;
use actix_web::{test, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use askama::Template;

#[derive(Template)]
#[template(path = "dashboard.html")]
struct Dashboard {
    title: &'static str,
    current_page: &'static str,
}

#[derive(Template)]
#[template(path = "user.html")]
struct UserTemplate {
    name: String,
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    title: &'static str,
    current_page: &'static str,
    editor_count: u32,
    lang_count: u32,
}

#[derive(Template)]
#[template(path = "login.html")]
struct Login {
    title: &'static str,
    current_page: &'static str,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginCredentials {
    email: String,
    password: String,
}

const ADDR: &'static str = "127.0.0.1:9190";

async fn index() -> Result<HttpResponse> {
    let index = Index {
        title: "CodingSesh · The Open Source Developer Dashboard",
        current_page: "home",
        editor_count: 0,
        lang_count: 0,
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(index))
}

async fn login() -> Result<HttpResponse> {
    let login = Login {
        title: "Login · CodingSesh",
        current_page: "login",
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(login))
}

async fn dashboard() -> Result<HttpResponse> {
    let db = Dashboard {
        title: "Your Dashboard · CodingSesh",
        current_page: "dashboard",
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(db))
}

/// Simple handle POST request
async fn handle_post_1(params: web::Form<LoginCredentials>) -> Result<HttpResponse> {
    let user = UserTemplate {
        name: params.email.to_string(),
        text: params.password.to_string(),
    }
    .render()
    .expect("Error: Failed to Render user.html");
    Ok(HttpResponse::Ok().content_type("text/html").body(user))
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/login").route(web::get().to(login)))
            .service(web::resource("/dashboard").route(web::get().to(dashboard)))
            .service(fs::Files::new("", "static").show_files_listing())
            .service(web::resource("/post1").route(web::post().to(handle_post_1))),
    );
}

#[actix_rt::main]
async fn main() {
    info(true);
    // start http server
    HttpServer::new(move || App::new().configure(app_config))
        .bind(ADDR)
        .expect("Error: Failed to bind Address")
        .start()
        .await;
}

#[cfg(target_family = "unix")]
fn info(debug: bool) {
    use termion::color;
    if debug {
        println!("{}    🔧  Configured for debug", color::Fg(color::Blue));
        println!(
            "{}    => {}Max Concurrent Connections per Worker: {}25k",
            color::Fg(color::White),
            color::Fg(color::Blue),
            color::Fg(color::White),
        );
        println!(
            "{}    => {}Max Concurrent Connections per Worker Rate: {}256",
            color::Fg(color::White),
            color::Fg(color::Blue),
            color::Fg(color::White),
        );
        println!(
            "{}    => {}Client Timeout: {}5s",
            color::Fg(color::White),
            color::Fg(color::Blue),
            color::Fg(color::White),
        );
        println!(
            "{}    => {}Client Shutdown: {}5s",
            color::Fg(color::White),
            color::Fg(color::Blue),
            color::Fg(color::White),
        );
        println!(
            "{}    => {}Shutdown Timeout: {}30s",
            color::Fg(color::White),
            color::Fg(color::Blue),
            color::Fg(color::White),
        );
        println!(
            "{}    => {}Workers: {}12",
            color::Fg(color::White),
            color::Fg(color::Blue),
            color::Fg(color::White),
        );
    } else {
        println!(
            "{}    🔧  Configured for production",
            color::Fg(color::Blue)
        );
    }
    println!(
        "{}    => {}Serving On: {}http://{}/",
        color::Fg(color::White),
        color::Fg(color::Blue),
        color::Fg(color::White),
        ADDR
    )
}
