use db;
use iron::headers::ContentType;
use iron::prelude::*;
use iron::status;
use models::{like, playlist, provider, track, user};
use params::{Params, Value};
use rustc_serialize::json;

fn compose_json_response<T: rustc_serialize::Encodable>(obj: &T) -> Response {
    let payload = json::encode(obj).unwrap();
    Response::with((ContentType::json().0, status::Ok, payload))
}

fn read_tracks(s: &String) -> Result<Vec<(String, String)>, String> {
    s.split(",")
        .map(|t| {
            let v: Vec<_> = t.split(":").collect();
            if v.len() == 2 {
                Ok((v[0].to_string(), v[1].to_string()))
            } else {
                Err("invalid query".to_string())
            }
        }).collect()
}

fn read_vec<T: std::str::FromStr>(s: &String) -> Result<Vec<T>, String> {
    s.split(",")
        .map(|e| e.parse().map_err(|_| "invalid query".to_string()))
        .collect()
}

pub fn add_user(req: &mut Request) -> IronResult<Response> {
    let pool = &req.get::<persistent::Read<db::DbPool>>().unwrap();
    let mut conn = pool.get().unwrap();
    let map = req.get_ref::<Params>().unwrap();
    match map.find(&["email"]) {
        Some(&Value::String(ref email)) => match user::create_user(&mut conn, &email) {
            Ok(_) => Ok(Response::with(status::Ok)),
            Err(e) => Ok(Response::with((status::InternalServerError, e))),
        },
        _ => Ok(Response::with((
            status::BadRequest,
            "query-params not found",
        ))),
    }
}

pub fn get_user(req: &mut Request) -> IronResult<Response> {
    let pool = &req.get::<persistent::Read<db::DbPool>>().unwrap();
    let mut conn = pool.get().unwrap();
    let map = req.get_ref::<Params>().unwrap();
    match map.find(&["email"]) {
        Some(&Value::String(ref email)) => match user::get_user(&mut conn, &email) {
            Ok(u) => Ok(compose_json_response(&u)),
            Err(e) => Ok(Response::with((status::InternalServerError, e))),
        },
        _ => Ok(Response::with((
            status::BadRequest,
            "query-params not found",
        ))),
    }
}

pub fn get_provider(req: &mut Request) -> IronResult<Response> {
    let pool = &req.get::<persistent::Read<db::DbPool>>().unwrap();
    let mut conn = pool.get().unwrap();
    let map = req.get_ref::<Params>().unwrap();
    match map.find(&["provider"]) {
        Some(&Value::String(ref provider)) => match provider::get_provider(&mut conn, &provider) {
            Ok(p) => Ok(compose_json_response(&p)),
            Err(e) => Ok(Response::with((status::InternalServerError, e))),
        },
        _ => Ok(Response::with((
            status::BadRequest,
            "query-params not found",
        ))),
    }
}

pub fn add_tracks(req: &mut Request) -> IronResult<Response> {
    let pool = &req.get::<persistent::Read<db::DbPool>>().unwrap();
    let mut conn = pool.get().unwrap();
    let map = req.get_ref::<Params>().unwrap();
    match map.find(&["tracks"]) {
        Some(&Value::String(ref ts)) => match read_tracks(&ts) {
            Ok(tracks) => match track::add_tracks(&mut conn, &tracks) {
                Ok(_) => Ok(Response::with(status::Ok)),
                Err(e) => Ok(Response::with((status::InternalServerError, e))),
            },
            Err(_) => Ok(Response::with(status::BadRequest)),
        },
        _ => Ok(Response::with(status::BadRequest)),
    }
}

pub fn add_likes(req: &mut Request) -> IronResult<Response> {
    let pool = &req.get::<persistent::Read<db::DbPool>>().unwrap();
    let mut conn = pool.get().unwrap();
    let map = req.get_ref::<Params>().unwrap();
    let email = match map.find(&["email"]) {
        Some(&Value::String(ref e)) => Some(format!("{}", e)),
        _ => None,
    };
    let tracks = match map.find(&["tracks"]) {
        Some(&Value::String(ref t)) => read_tracks(&t).ok(),
        _ => None,
    };
    if email.is_none() || tracks.is_none() {
        Ok(Response::with(status::BadRequest))
    } else {
        let ts = tracks.unwrap();
        match track::add_tracks(&mut conn, &ts)
            .and_then(|_| like::add_likes(&mut conn, &email.unwrap(), &ts))
        {
            Ok(_) => Ok(Response::with(status::Ok)),
            Err(e) => Ok(Response::with((status::InternalServerError, e))),
        }
    }
}

pub fn del_likes(req: &mut Request) -> IronResult<Response> {
    let pool = &req.get::<persistent::Read<db::DbPool>>().unwrap();
    let mut conn = pool.get().unwrap();
    let map = req.get_ref::<Params>().unwrap();
    let email = match map.find(&["email"]) {
        Some(&Value::String(ref e)) => Some(format!("{}", e)),
        _ => None,
    };
    let tracks = match map.find(&["tracks"]) {
        Some(&Value::String(ref t)) => read_tracks(&t).ok(),
        _ => None,
    };
    if email.is_none() || tracks.is_none() {
        Ok(Response::with(status::BadRequest))
    } else {
        match like::del_likes(&mut conn, &email.unwrap(), &tracks.unwrap()) {
            Ok(_) => Ok(Response::with(status::Ok)),
            Err(e) => Ok(Response::with((status::InternalServerError, e))),
        }
    }
}

pub fn get_likes(req: &mut Request) -> IronResult<Response> {
    let pool = &req.get::<persistent::Read<db::DbPool>>().unwrap();
    let mut conn = pool.get().unwrap();
    let map = req.get_ref::<Params>().unwrap();
    match map.find(&["email"]) {
        Some(&Value::String(ref email)) => match like::get_likes(&mut conn, &email) {
            Ok(ts) => Ok(compose_json_response(&ts)),
            Err(e) => Ok(Response::with((status::InternalServerError, e))),
        },
        _ => Ok(Response::with((
            status::BadRequest,
            "query-params not found",
        ))),
    }
}

pub fn add_playlist(req: &mut Request) -> IronResult<Response> {
    let pool = &req.get::<persistent::Read<db::DbPool>>().unwrap();
    let mut conn = pool.get().unwrap();
    let map = req.get_ref::<Params>().unwrap();
    let email = match map.find(&["email"]) {
        Some(&Value::String(ref e)) => Some(format!("{}", e)),
        _ => None,
    };
    let name = match map.find(&["name"]) {
        Some(&Value::String(ref n)) => Some(format!("{}", n)),
        _ => None,
    };
    if email.is_none() || name.is_none() {
        Ok(Response::with(status::BadRequest))
    } else {
        match playlist::add_playlist(&mut conn, &email.unwrap(), &name.unwrap()) {
            Ok(p) => Ok(compose_json_response(&p)),
            Err(e) => Ok(Response::with((status::InternalServerError, e))),
        }
    }
}

pub fn del_playlist(req: &mut Request) -> IronResult<Response> {
    let pool = &req.get::<persistent::Read<db::DbPool>>().unwrap();
    let mut conn = pool.get().unwrap();
    let map = req.get_ref::<Params>().unwrap();
    let email = match map.find(&["email"]) {
        Some(&Value::String(ref e)) => Some(format!("{}", e)),
        _ => None,
    };
    let token = match map.find(&["token"]) {
        Some(&Value::String(ref t)) => Some(format!("{}", t)),
        _ => None,
    };
    if email.is_none() || token.is_none() {
        Ok(Response::with(status::BadRequest))
    } else {
        match playlist::del_playlist(&mut conn, &email.unwrap(), &token.unwrap()) {
            Ok(_) => Ok(Response::with(status::Ok)),
            Err(e) => Ok(Response::with((status::InternalServerError, e))),
        }
    }
}

pub fn get_playlists(req: &mut Request) -> IronResult<Response> {
    let pool = &req.get::<persistent::Read<db::DbPool>>().unwrap();
    let mut conn = pool.get().unwrap();
    let map = req.get_ref::<Params>().unwrap();
    match map.find(&["email"]) {
        Some(&Value::String(ref email)) => match playlist::get_playlists(&mut conn, &email) {
            Ok(ps) => Ok(compose_json_response(&ps)),
            Err(e) => Ok(Response::with((status::InternalServerError, e))),
        },
        _ => Ok(Response::with((
            status::BadRequest,
            "query-params not found",
        ))),
    }
}

pub fn add_playlistitems(req: &mut Request) -> IronResult<Response> {
    // TODO: プレイリストアイテムの順序付け，重複を許す
    let pool = &req.get::<persistent::Read<db::DbPool>>().unwrap();
    let mut conn = pool.get().unwrap();
    let map = req.get_ref::<Params>().unwrap();
    let email = match map.find(&["email"]) {
        Some(&Value::String(ref e)) => Some(format!("{}", e)),
        _ => None,
    };
    let token = match map.find(&["token"]) {
        Some(&Value::String(ref t)) => Some(format!("{}", t)),
        _ => None,
    };
    let tracks = match map.find(&["tracks"]) {
        Some(&Value::String(ref ts)) => read_tracks(&ts).ok(),
        _ => None,
    };
    if email.is_none() || token.is_none() || tracks.is_none() {
        Ok(Response::with(status::BadRequest))
    } else {
        let ts = tracks.unwrap();
        match track::add_tracks(&mut conn, &ts).and_then(|_| {
            playlist::add_playlistitems(&mut conn, &email.unwrap(), &token.unwrap(), &ts)
        }) {
            Ok(_) => Ok(Response::with(status::Ok)),
            Err(e) => Ok(Response::with((status::InternalServerError, e))),
        }
    }
}

pub fn get_playlistitems(req: &mut Request) -> IronResult<Response> {
    let pool = &req.get::<persistent::Read<db::DbPool>>().unwrap();
    let mut conn = pool.get().unwrap();
    let map = req.get_ref::<Params>().unwrap();
    let email = match map.find(&["email"]) {
        Some(&Value::String(ref e)) => Some(format!("{}", e)),
        _ => None,
    };
    let token = match map.find(&["token"]) {
        Some(&Value::String(ref t)) => Some(format!("{}", t)),
        _ => None,
    };
    if email.is_none() || token.is_none() {
        Ok(Response::with(status::BadRequest))
    } else {
        match playlist::get_playlistitems(&mut conn, &email.unwrap(), &token.unwrap()) {
            Ok(ts) => Ok(compose_json_response(&ts)),
            Err(e) => Ok(Response::with((status::InternalServerError, e))),
        }
    }
}

pub fn del_playlistitems(req: &mut Request) -> IronResult<Response> {
    let pool = &req.get::<persistent::Read<db::DbPool>>().unwrap();
    let mut conn = pool.get().unwrap();
    let map = req.get_ref::<Params>().unwrap();
    let email = match map.find(&["email"]) {
        Some(&Value::String(ref e)) => Some(format!("{}", e)),
        _ => None,
    };
    let token = match map.find(&["token"]) {
        Some(&Value::String(ref t)) => Some(format!("{}", t)),
        _ => None,
    };
    let ranks = match map.find(&["ranks"]) {
        Some(&Value::String(ref rs)) => read_vec::<i32>(&rs).ok(),
        _ => None,
    };
    if email.is_none() || token.is_none() || ranks.is_none() {
        Ok(Response::with(status::BadRequest))
    } else {
        let rs = ranks.unwrap();
        match playlist::del_playlistitems(&mut conn, &email.unwrap(), &token.unwrap(), &rs) {
            Ok(_) => Ok(Response::with(status::Ok)),
            Err(e) => Ok(Response::with((status::InternalServerError, e))),
        }
    }
}
