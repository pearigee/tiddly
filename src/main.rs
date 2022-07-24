#[macro_use]
extern crate rocket;
use rocket::data::{Data, ToByteUnit};
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::State;

struct TiddlyConfig {
    path: String,
    backup_dir: String,
    version: String,
}

fn backup(config: &TiddlyConfig) -> Result<(), std::io::Error> {
    let filename = chrono::Local::now()
        .format("%Y-%m-%d-%H-%M-%S.html")
        .to_string();
    let backup_path: String = format!(
        "{}{}{}",
        &config.backup_dir,
        std::path::MAIN_SEPARATOR,
        filename
    );

    std::fs::copy(&config.path, backup_path)?;

    Ok(())
}

#[get("/")]
async fn index(config: &State<TiddlyConfig>) -> Option<NamedFile> {
    println!("Serving wiki: {}", &config.path);
    NamedFile::open(&config.path).await.ok()
}

#[put("/", data = "<content>")]
async fn save(config: &State<TiddlyConfig>, content: Data<'_>) -> (Status, &'static str) {
    println!("Backing up wiki to: {}", &config.backup_dir);
    if backup(&config).is_err() {
        return (Status::InternalServerError, "Failed to backup wiki");
    }

    println!("Saving wiki: {}", &config.path);
    content
        .open(100.megabytes())
        .into_file(&config.path)
        .await
        .map(|_| (Status::Ok, "Wiki backedup and saved successfully"))
        .unwrap_or((Status::InternalServerError, "Failed to save file"))
}

struct DavOptions {
    server_version: String,
}

impl<'a> Responder<'a, 'a> for DavOptions {
    fn respond_to(self, _: &Request) -> response::Result<'a> {
        Response::build()
            .raw_header("dav", "tw5/put")
            .raw_header("tiddly-server-version", self.server_version)
            .ok()
    }
}

// This is how TW determines what save options are available.
// The 'dav' header indicates the wiki can be saved with a PUT.
#[options("/")]
fn options(config: &State<TiddlyConfig>) -> DavOptions {
    DavOptions {
        server_version: config.version.to_string(),
    }
}

// TW calls HEAD after a save. We don't need to take any action so we just
// return a successful status code.
#[head("/")]
fn head() -> Status {
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    let suggested_usage = "`tiddly [target.html] [backup directory]`";

    if std::env::args().len() < 3 {
        println!("Usage: {}", suggested_usage);
        std::process::exit(1);
    }

    let target = std::env::args()
        .nth(1)
        .expect(&format!("No target html file defined! {}", suggested_usage));

    let backup_dir = std::env::args().nth(2).expect(&format!(
        "No target backup directory defined! {}",
        suggested_usage
    ));

    println!("Running tiddly server at http://127.0.0.1:8000");
    rocket::build()
        .mount("/", routes![index, save, options, head])
        .manage(TiddlyConfig {
            path: target.to_string(),
            backup_dir: backup_dir.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        })
}
