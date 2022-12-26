use mysql::prelude::{FromRow, Queryable};

use crate::web;




pub fn query_info<T:FromRow, D: FnMut(T) -> U, U>(script: &str, d: D) -> mysql::Result<Vec<U>>
// 可变参数函数 返回类型 U
{
    let pool = web::database_connection();

    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => panic!("{}", e),
    };

    conn.query_map(script, d)
}
