#![allow(non_snake_case)]

use mysql::chrono::Utc;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Track {
    pub provider: String,
    pub contentId: String,
}

pub fn add_tracks<'a>(
    conn: &'a mut mysql::Conn,
    tracks: &Vec<(String, String)>,
) -> Result<String, String> {
    let mut isok = true;
    for mut stmt in conn
        .prepare(
            r"insert into tracks (created_at, provider_id, content_id)
              select :created_at, provider_id, content_id
              from (
                select p.id provider_id, :content_id content_id
                from providers p
                where p.name=:provider
              ) pc
              where not exists (
                select * from tracks t where t.provider_id=pc.provider_id and t.content_id=pc.content_id
              )"
        ).into_iter() {
        for t in tracks {
            isok = stmt
                .execute(params! {
                    "created_at" => Utc::now().naive_utc(),
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
