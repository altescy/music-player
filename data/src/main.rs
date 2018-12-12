extern crate iron;
#[macro_use]
extern crate mysql;
extern crate params;
extern crate persistent;
extern crate r2d2;
extern crate r2d2_mysql;
#[macro_use]
extern crate router;
extern crate rustc_serialize;

mod controllers;
mod models;
mod utils;

use controllers::{db, handler};
use iron::prelude::*;
use std::env;

fn get_env(key: &str, def: &str) -> String {
    env::var(key).ok().unwrap_or(def.to_string())
}

fn main() {
    let port = get_env("DATA_API_PORT", "8080");
    let dbhost = get_env("DB_HOST", "localhost");
    let dbport = get_env("DB_PORT", "3306");
    let dbname = get_env("MYSQL_DATABASE", "mysql");
    let dbuser = get_env("MYSQL_USER", "user");
    let dbpass = get_env("MYSQL_PASSWORD", "password");

    let mut builder = mysql::OptsBuilder::new();
    builder
        .ip_or_hostname(Some(dbhost))
        .tcp_port(dbport.parse::<u16>().unwrap())
        .user(Some(dbuser))
        .pass(Some(dbpass))
        .db_name(Some(dbname));
    let manager = r2d2_mysql::MysqlConnectionManager::new(builder);

    // TODO: deletedなデータを取得できるようにする
    let router = router!(
        add_user:          post   "/user"            => handler::add_user,            // user?email=[email]
        get_user:          get    "/user"            => handler::get_user,            // user?email=[email]
        get_provider:      get    "/provider"        => handler::get_provider,        // provider?provider=[provider]
        add_tracks:        post   "/tracks"          => handler::add_tracks,          // tracks?tracks=[p1:c1],[p2:c2],...
        add_like:          post   "/likes"           => handler::add_likes,           // like?email=[email]&track=[p:c]
        del_like:          delete "/likes"           => handler::del_likes,           // like?email=[email]&track=[p:c]
        get_likes:         get    "/likes"           => handler::get_likes,           // likes?email=[email]
        add_playlist:      post   "/playlist"        => handler::add_playlist,        // playlist?email=[email]&name=[name] -> token
        del_playlist:      delete "/playlist"        => handler::del_playlist,        // playlist?email=[email]&token=[token]
        get_playlists:     get    "/playlists"       => handler::get_playlists,       // playlists?email=[email]
        add_playlistimtes: post   "/playlist/tracks" => handler::add_playlistitems,   // playlist/tracks?email=[email]&token=[token]&tracks=[p:c],...
        get_playlistitems: get    "/playlist/tracks" => handler::get_playlistitems,   // playlist/tracks?email=[email]&token=[token]
        del_plailistitems: delete "/playlist/tracks" => handler::del_playlistitems,   // playlist/tracks?email=[email]&token=[token]&ranks=[r],...
    );

    let mut chain = Chain::new(router);
    chain.link(persistent::Read::<db::DbPool>::both(
        r2d2::Pool::builder().max_size(15).build(manager).unwrap(),
    ));

    println!("listen on 0.0.0.0:{}", &port);
    Iron::new(chain).http(format!("0.0.0.0:{}", port)).unwrap();
}
