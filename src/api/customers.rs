use serde_json;

use error::HelpScoutError;
use client::{Client, Status};

const PREFIX: &'static str = "mailboxes";

#[derive(Debug, Deserialize)]
pub struct ItemList<T> {
    page: i32,
    pages: i32,
    count: i32,
    items: Vec<T>,
}

//Class representing possible values returned for the customers endpoint for the customer object
#[derive(Debug, Deserialize)]
pub struct Customer {
    id: String,
    firstName: String,
    lastName: String,
    photoUrl: String,
    photoType: String,
    gender: String,
    age: String,
    organization:String,
    jobTitle: String,
    location: String,
    createdAt: String,
    modifiedAt: String,
}

pub fn list(client: &Client, mailbox_id: &str) -> Result<(Status, ItemList<Customer>), HelpScoutError> {
    let (status, res) = client.get(PREFIX, &format!("{}/customers.json", mailbox_id), None)?;

    println!("status {:?}", status);
    println!("result - {:?}", res);
    let customers = serde_json::from_value(res.clone())?;

    Ok((status, customers))
}