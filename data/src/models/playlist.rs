#![allow(non_snake_case)]
use models::track::Track;
use mysql::chrono::Utc;
use utils;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Playlist {
    pub token: String,
    pub name: String,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct PlaylistItem {
    pub provider: String,
    pub contentId: String,
    pub rankorder: i32,
}

#[derive(Hash)]
struct PlaylistTokenSeed {
    email: String,
    name: String,
    datetime: String,
}

pub fn add_playlist<'a>(
    conn: &'a mut mysql::Conn,
    email: &String,
    name: &String,
) -> Result<Playlist, String> {
    let token = utils::gen_token(&PlaylistTokenSeed {
        email: format!("{}", &email),
        name: format!("{}", &name),
        datetime: format!("{:?}", Utc::now().naive_utc()),
    });
    conn.prep_exec(
        r"insert into playlists (created_at, updated_at, user_id, name, token)
          select :created_at, :created_at, u.id, :name, :token
          from (
            select id from users where email=:email
          ) u
          where not exists (
            select * from playlists p where p.user_id=u.id and p.token=:token
          )",
        params! {
            "created_at" => Utc::now().naive_utc(),
            "email" => &email,
            "name" => &name,
            "token" => &token,
        },
    ).map_err(|e| e.to_string())
    .and_then(|_| {
        Ok(Playlist {
            token: token,
            name: format!("{}", name),
        })
    })
}

pub fn del_playlist<'a>(
    conn: &'a mut mysql::Conn,
    email: &String,
    token: &String,
) -> Result<mysql::QueryResult<'a>, String> {
    conn.prep_exec(
        r"update playlists set deleted_at=:deleted_at where user_id=(select id from users where email=:email) and token=:token",
        params! {
            "deleted_at" => Utc::now().naive_utc(),
            "email" => email,
            "token" => token,
        },
    ).map_err(|e| e.to_string())
}

pub fn get_playlists<'a>(
    conn: &'a mut mysql::Conn,
    email: &String,
) -> Result<Vec<Playlist>, String> {
    conn.prep_exec(
        r"select token, name from playlists p inner join (select id from users where email=:email) u on p.user_id=u.id where p.deleted_at is null",
        params! { "email" => &email, },
    ).map(|result| {
        result
            .map(|x| x.unwrap())
            .map(|row| {
                let (token, name) = mysql::from_row(row);
                Playlist {
                    token: token,
                    name: name
                }
            })
            .collect()
    }).map_err(|e| e.to_string())
}

pub fn add_playlistitems<'a>(
    conn: &'a mut mysql::Conn,
    email: &String,
    token: &String,
    tracks: &Vec<(String, String)>,
) -> Result<String, String> {
    let mut isok = true;
    for mut stmt in conn
        .prepare(
            r"insert into playlistitems (created_at, updated_at, playlist_id, track_id, rankorder)
              select :created_at, :created_at, p.id, t.id, cnt
              from (
                select id
                from playlists
                where
                  user_id=(select id from users where email=:email)
                  and token=:token
                  and deleted_at is null
              ) p, (
                select id
                from tracks
                where
                  provider_id=(select id from providers where name=:provider)
                  and content_id=:content_id
              ) t, (
                select count(*) cnt
                from playlistitems
                where
                  playlist_id=(select id from playlists where token=:token)
              ) c
              limit 1
            ",
        ).into_iter()
    {
        for t in tracks {
            isok = stmt
                .execute(params! {
                    "created_at" => Utc::now().naive_utc(),
                    "email" => &email,
                    "token" => &token,
                    "provider" => &t.0,
                    "content_id" => &t.1,
                }).is_ok();
        }
    }
    if isok {
        Ok("success to insert tracks".to_string())
    } else {
        Err("fail to insert some tracks".to_string())
    }
}

pub fn get_playlistitems<'a>(
    conn: &'a mut mysql::Conn,
    email: &String,
    token: &String,
) -> Result<Vec<PlaylistItem>, String> {
    conn.prep_exec(
        r"select name, content_id, rankorder
          from (
            select provider_id, content_id, rankorder
            from tracks t inner join (
              select track_id, rankorder
              from playlistitems
              where
                playlist_id=(
                  select id
                  from playlists
                  where
                    token=:token
                    and user_id=(select id from users where email=:email)
                    and deleted_at is null)
                and deleted_at is null
            ) p on t.id=p.track_id) tp
            inner join providers pr on tp.provider_id = pr.id
          order by rankorder",
        params! {
            "email" => &email,
            "token" => &token,
        },
    ).map(|result| {
        result
            .map(|x| x.unwrap())
            .map(|row| {
                let (provider, content_id, rankorder) = mysql::from_row(row);
                PlaylistItem {
                    provider: provider,
                    contentId: content_id,
                    rankorder: rankorder,
                }
            }).collect()
    }).map_err(|e| e.to_string())
}

pub fn del_playlistitems<'a>(
    conn: &'a mut mysql::Conn,
    email: &String,
    token: &String,
    ranks: &Vec<i32>,
) -> Result<String, String> {
    if conn.prepare(
        r"update
          playlistitems pi,
          (select id from playlists where user_id=(select id from users where email=:email) and token=:token) p
        set
          pi.updated_at = :deleted_at,
          pi.deleted_at = :deleted_at
        where
          pi.playlist_id = p.id and pi.rankorder=:rankorder",
    ).into_iter().map(|mut stmt| {
        ranks.iter().map(move |r|
            stmt.execute(params! {
                "deleted_at" => Utc::now().naive_utc(),
                "email" => &email,
                "token" => &token,
                "rankorder" => &r,
            }).map_err(|_| "fail to delete playlistitems".to_string())
                .and_then(|_| Ok("success to delete playlistitems".to_string()))
        )
    }).flatten().any(|e| e.is_err()) {
        Err("fail to delete playlistitems".to_string())
    } else {
        Ok("success to delete playlistitems".to_string())
    }
}
