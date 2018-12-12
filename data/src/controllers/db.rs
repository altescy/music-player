#[derive(Copy, Clone)]
pub struct DbPool;
impl iron::typemap::Key for DbPool {
    type Value = r2d2::Pool<r2d2_mysql::MysqlConnectionManager>;
}
