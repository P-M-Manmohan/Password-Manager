use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Credentials{
    //pub id: i32,
   // pub service: String,
    pub username: String,
    pub password: String,
    pub nonce_user: String,
    pub nonce_pass: String,
}
