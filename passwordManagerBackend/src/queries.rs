use sqlx::{query,PgPool};
use crate::models::credentials::{Services, Credentials};

pub async fn list_services(
    db_pool: PgPool,
) -> Result<serde_json::Value, String> {
    let response = sqlx::query_as::<_,Services>("SELECT service FROM passwords")
    .fetch_all(&db_pool)
    .await;

    match response{
        Ok(res) =>{ match serde_json::to_value(res){
                Ok(out) => Ok(out),
                Err(err) => Err(format!("json error {}", err)),
            }
        }
        Err(err) => {
            Err(format!("database error {}",err))
        }
    }

}

pub async fn add_password(
    db_pool: PgPool,
    password:  &String,
    username: &String,
    service: &String,
    nonce_user: &String,
    nonce_pass: &String
)-> Result<(), sqlx::Error> {
    let response = query!("INSERT INTO passwords (service, username, password, nonce_pass, nonce_user) VALUES ($1,$2,$3,$4,$5)", service, username, password, nonce_pass, nonce_user)
        .execute(&db_pool)
        .await;

    match response{
        Ok(_status) => Ok(()),
        Err(err) => {
            eprintln!("Database error: {err}");
            Err(err)
        }

    }
}

pub async fn get_credentials(
    db_pool: PgPool,
    service: &String
) -> Result<serde_json::Value,String> {
    let response = sqlx::query_as::<_,Credentials>("SELECT username, password, nonce_pass, nonce_user FROM passwords WHERE service = $1")
        .bind(service)
        .fetch_one(&db_pool)
        .await;

    match response{
        Ok(res) =>{ match serde_json::to_value(res){
                Ok(out) => Ok(out),
                Err(err) => Err(format!("json error {}", err)),
            }
        }
        Err(err) => {
            Err(format!("database error {}",err))
        }
    }
}

pub async fn delete_credentials(
    db_pool: PgPool,
    service: &String
) -> Result<(), sqlx::Error>{
    println!("{}",service);
    let response = sqlx::query!("DELETE FROM passwords WHERE service = $1", service)
    .execute(&db_pool)
    .await;

    match response{
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("Database error {}", err);
            Err(err)
        }
    }
}
