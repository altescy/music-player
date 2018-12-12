use mysql::chrono::{NaiveDateTime, Utc};

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct User {
    pub id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub email: String,
}

pub fn create_user<'a>(
    conn: &'a mut mysql::Conn,
    email: &String,
) -> Result<mysql::QueryResult<'a>, String> {
    conn.prep_exec(
        r"insert into users (created_at, updated_at, email)
          select :updated_at, :updated_at, :email
          on duplicate key update
            users.updated_at = :updated_at",
        params! {
            "updated_at" => Utc::now().naive_utc(),
            "email" => email,
        },
    ).map_err(|e| e.to_string())
}

pub fn get_user<'a>(conn: &'a mut mysql::Conn, email: &String) -> Result<User, String> {
    let mut results: Vec<User> = conn
        .prep_exec(
            "select id, created_at, email from users where email = :email limit 1",
            params! { "email" => &email, },
        ).map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (id, created_at, email) = mysql::from_row(row);
                    User {
                        id: id,
                        created_at: created_at,
                        email: email,
                    }
                }).collect()
        }).unwrap();
    results.pop().ok_or("user not found".to_string())
}
