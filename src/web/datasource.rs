use mysql::{
    prelude::{AsStatement, FromRow, Queryable},
    Params,
};

use crate::web;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

pub fn query_info<T, D, U>(script: &str, d: D) -> mysql::Result<Vec<U>>
where
    T: FromRow,
    D: FnMut(T) -> U,
    // 可变参数函数 返回类型 U
{
    let pool = web::database_connection();

    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => panic!("{}", e),
    };

    conn.query_map(script, d)
}


pub fn _insert_data<S, P, I>(stmt: S, param: I)
where
    S: AsStatement,
    P: Into<Params>,
    I: IntoIterator<Item = P>,
{
    let pool = web::database_connection();

    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => panic!("{}", e),
    };
    conn.exec_batch(stmt, param).unwrap();
}
