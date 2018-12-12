use models::track::Track;
use mysql::chrono::Utc;

pub fn add_likes<'a>(
    conn: &'a mut mysql::Conn,
    email: &String,
    tracks: &Vec<(String, String)>,
) -> Result<String, String> {
    if conn.prepare(
        r"insert into likes (created_at, updated_at, user_id, track_id)
          select :updated_at, :updated_at, u.id, t.id
          from
            (select id from users where email=:email) u,
            (select id from tracks where provider_id=(select id from providers where name=:provider) and content_id=:content_id) t
          limit 1
          on duplicate key update
            likes.updated_at = :updated_at,
            likes.deleted_at = null",
    ).into_iter().map(|mut stmt| {
        tracks.iter().map(move |t|
            stmt.execute(params! {
                "updated_at" => Utc::now().naive_utc(),
                "email" => &email,
                "provider" => &t.0,
                "content_id" => &t.1,
            }).is_err()
        )
    }).flatten().any(|e| e) {
        Err("fail to add likes".to_string())
    } else {
        Ok("success to add likes".to_string())
    }
}

pub fn del_likes<'a>(
    conn: &'a mut mysql::Conn,
    email: &String,
    tracks: &Vec<(String, String)>,
) -> Result<String, String> {
    if conn.prepare(
        r"update
            likes l,
            (select id from users where email=:email) u,
            (select id from tracks where provider_id=(select id from providers where name=:provider) and content_id=:content_id) t
          set
            l.updated_at = :deleted_at,
            l.deleted_at = :deleted_at
          where
            l.user_id = u.id
            and l.track_id = t.id",
    ).into_iter().map(|mut stmt| {
        tracks.iter().map(move |t|
            stmt.execute(params! {
                "deleted_at" => Utc::now().naive_utc(),
                "email" => &email,
                "provider" => &t.0,
                "content_id" => &t.1,
            }).is_err()
        )
    }).flatten().any(|e| e) {
        Err("fail to delete likes".to_string())
    } else {
        Ok("success to delete likes".to_string())
    }
}

pub fn get_likes<'a>(conn: &'a mut mysql::Conn, email: &String) -> Result<Vec<Track>, String> {
    conn.prep_exec(
        r"select p.name, content_id
          from (
            select provider_id, content_id
            from tracks
            where tracks.id in (
              select track_id
              from likes
              where user_id=(select id from users where email=:email) and deleted_at is null)) t
            inner join providers p on t.provider_id=p.id",
        params! { "email" => &email, },
    ).map(|result| {
        result
            .map(|x| x.unwrap())
            .map(|row| {
                let (provider, content_id) = mysql::from_row(row);
                Track {
                    provider: provider,
                    contentId: content_id,
                }
            }).collect()
    }).map_err(|e| e.to_string())
}
