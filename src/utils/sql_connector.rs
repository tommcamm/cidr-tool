// sql related stuff will be here

use mysql::{OptsBuilder, Pool, PooledConn};
use mysql::prelude::Queryable;

pub fn write_to_mysql(mut conn :PooledConn, table :&str) -> mysql::Result<()> {
    // create the table using a very efficient store format [only for probationary period]
    conn.query_drop(r"CREATE TABLE cidrip (
                                ip unsigned int not null)")?;

    Ok(())
}

pub fn mysql_create_conn(db_user :&str, db_pass :&str,
                         db_name :&str, db_uri :&str) -> mysql::Result<PooledConn> {

    let builder = OptsBuilder::new()
        .ip_or_hostname(Some(db_uri))
        .db_name(Some(db_name))
        .user(Some(db_user))
        .pass(Some(db_pass));

    let pool = Pool::new(builder)?;
    pool.get_conn()
}
