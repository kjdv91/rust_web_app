use sqlx::{Pool, Postgres};
use sqlx::time::Duration;
use sqlx::postgress::PgPoolOptions;



const PG_HOST: &str = "localhost";
const PG_ROOT_DB : &str = "postgres";
const PG_HOST_USER: &str = "postgres";
const PG_ROOT_PWD : &str = "postgres";


pub type Db = Pool<Postgres>;


pub async fn init_db() -> Result<Db, sqlx::Error> {
    new_db_poll(PG_HOST, PG_ROOT_DB, PG_HOST_USER, PG_ROOT_PWD, 1).await
}


// function connecct
async fn new_db_poll(host: &str, db : &str, user: &str, pwd: &str, max_con: u32) -> Result<Db, sqlx::Error>{
    let con_string = format!("postgress:// {} : {} @ {}/{}", user, pwd, host, db);

    PgPoolOptions::new()
    .max_connections(max_con)
    .connect_timeout(Duration::from_millis(500))
    .connect(&*con_string)
    .await  // connect to the database

}


#[cfg(test)]
#[path= "../_tests/model_db.rs"]
mod test; 
