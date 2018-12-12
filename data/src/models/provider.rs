#[derive(Debug, RustcEncodable)]
pub struct Provider {
    id: Option<i32>,
    name: String,
}

pub fn get_provider<'a>(conn: &'a mut mysql::Conn, provider: &String) -> Result<Provider, String> {
    let mut results: Vec<Provider> = conn
        .prep_exec(
            "select id, name from providers where name = :name limit 1",
            params! { "name" => &provider, },
        ).map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (id, name) = mysql::from_row(row);
                    Provider { id: id, name: name }
                }).collect()
        }).unwrap();
    results.pop().ok_or("provider not found".to_string())
}
