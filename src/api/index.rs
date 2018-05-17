use actix::*;
use actix_web::{HttpRequest, Result};
use actix_web::fs::NamedFile;
use std::path::Path;
use model::db::ConnDsl;

pub struct AppState {
    pub db: Addr<Syn, ConnDsl>
}

pub fn home(_req: HttpRequest<AppState>) -> Result<NamedFile> {
    Ok(NamedFile::open(Path::new("public/index.html"))?)
}

pub fn path(_req: HttpRequest<AppState>) -> Result<NamedFile> {
    Ok(NamedFile::open(Path::new("public/index.html"))?)
}
