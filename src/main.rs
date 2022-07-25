mod cli;

#[macro_use]
extern crate rocket;
use clap::Parser;
use rocket::data::{Data, ToByteUnit};
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::State;

use crate::cli::Args;

fn backup(config: &Args) -> Result<(), std::io::Error> {
    let filename = chrono::Local::now()
        .format("%Y-%m-%d-%H-%M-%S.html")
        .to_string();
    let backup_path: String = format!(
        "{}{}{}",
        &config.backup_dir.display(),
        std::path::MAIN_SEPARATOR,
        filename
    );

    std::fs::copy(&config.target, backup_path)?;

    Ok(())
}

#[get("/")]
async fn index(config: &State<Args>) -> Option<NamedFile> {
    println!("Serving wiki: {}", &config.target.display());
    NamedFile::open(&config.target).await.ok()
}

#[put("/", data = "<content>")]
async fn save(config: &State<Args>, content: Data<'_>) -> (Status, &'static str) {
    println!("Backing up wiki to: {}", &config.backup_dir.display());
    if backup(&config).is_err() {
        return (Status::InternalServerError, "Failed to backup wiki");
    }

    println!("Saving wiki: {}", &config.target.display());
    content
        .open(100.megabytes())
        .into_file(&config.target)
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
fn options() -> DavOptions {
    DavOptions {
        server_version: env!("CARGO_PKG_VERSION").to_string(),
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
    let args = Args::parse();

    println!("Running tiddly server at http://127.0.0.1:{}", args.port);

    let server_config = rocket::Config::figment().merge(("port", args.port));
    rocket::custom(server_config)
        .mount("/", routes![index, save, options, head])
        .manage(args)
}
