// sql related stuff will be here

use std::ops::Add;
use mysql::{Pool, PooledConn};
use mysql::prelude::Queryable;

pub fn write_to_mysql(mut conn :PooledConn, table :Str) -> Result<Ok, Err(E)> {
    // create the table using a very efficient store format [only for probationary period]
    conn.query_drop(r"CREATE TABLE cidrip (
                                ip unsigned int not null)")?;

    Ok(())
}

pub fn mysql_create_conn(db_user :Str, db_pass :Str,
                         db_name :Str, db_uri :Str) -> Result<Ok(T), Err(E)> {
    let url = String::from("mysql://").add(db_user).add(":").add(db_pass)
        .add("@").add(db_uri).add("/").add(db_name);

    let pool = Pool::new(url)?;
    pool.get_conn()
}
