use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]

pub struct SubscriberRequest{
    pub url: String,
    pub name: String,
}