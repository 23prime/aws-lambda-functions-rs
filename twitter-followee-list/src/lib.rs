use log::info;
use std::collections::HashSet;
use std::env;

use dotenv::dotenv;
use egg_mode::list::{List, ListID};
use egg_mode::{list, user, KeyPair, Token};

type BoxError = Box<dyn std::error::Error>;

pub mod error;
pub mod event;
pub mod logger;

// See Twitter API Reference Standard v1.1:
// https://developer.twitter.com/en/docs/twitter-api/v1

pub async fn run() -> Result<(), BoxError> {
    dotenv().ok();
    const PAGE_SIZE: i32 = 10000;
    const EACH_API_CALL_LIMIT_SIZE: usize = 100;

    let token = get_token();

    let me = get_me(&token).await?;
    info!("Get me: {:?}", me);

    let mut friends = get_friends(&token, PAGE_SIZE).await?;
    friends.insert(me); // Add me for insert me to list
    info!("Get current friends and me: {:?}", friends);

    let mut list_id = get_list_id();
    let list_result = list::show(list_id.clone(), &token).await;
    info!("Target list: {:?}", list_result);

    if list_result.is_err() {
        let created_list = create_list(&token).await?;
        list_id = ListID::from_id(created_list.id);
    }

    let list_members = get_list_members(list_id.clone(), &token, PAGE_SIZE).await?;
    info!("Target list members: {:?}", list_members);

    let excess = (&list_members - &friends).into_iter().collect::<Vec<_>>();
    info!("Excess: {:?}", get_user_names(&excess, &token).await);

    for partial_excess in split_vec(excess, EACH_API_CALL_LIMIT_SIZE) {
        let remove_result =
            list::remove_member_list(partial_excess.clone(), list_id.clone(), &token).await;
        info!(
            "Removing result: {:?} => {:?}",
            partial_excess, remove_result
        );
    }

    let deficiency = (&friends - &list_members).into_iter().collect::<Vec<_>>();
    info!(
        "Deficiency: {:?}",
        get_user_names(&deficiency, &token).await
    );

    for partial_deficiency in split_vec(deficiency, EACH_API_CALL_LIMIT_SIZE) {
        let add_result =
            list::add_member_list(partial_deficiency.clone(), list_id.clone(), &token).await;
        info!(
            "Adding result: {:?} => {:?}",
            partial_deficiency, add_result
        );
    }

    info!("Done!");

    return Ok(());
}

fn split_vec<T>(vec: Vec<T>, each_len: usize) -> Vec<Vec<T>> {
    let mut partial_vec = vec![];
    let mut result = vec![];

    for elem in vec {
        partial_vec.push(elem);

        if partial_vec.len() == each_len {
            result.push(partial_vec);
            partial_vec = vec![];
        }
    }

    if !partial_vec.is_empty() {
        result.push(partial_vec);
    }

    return result;
}

fn get_token() -> Token {
    let consumer_key = env::var("CONSUMER_KEY").expect("CONSUMER_KEY must be set");
    let consumer_secret = env::var("CONSUMER_SECRET").expect("CONSUMER_SECRET must be set");
    let access_key = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN must be set");
    let access_secret = env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET must be set");

    let con_token = KeyPair::new(consumer_key, consumer_secret);
    let access_token = egg_mode::KeyPair::new(access_key, access_secret);

    return Token::Access {
        consumer: con_token,
        access: access_token,
    };
}

async fn get_me(token: &Token) -> Result<u64, BoxError> {
    let user_id = env::var("USER_ID").expect("USER_ID must be set");
    return Ok(user::show(user_id, &token).await?.id);
}

async fn get_friends(token: &Token, page_size: i32) -> Result<HashSet<u64>, BoxError> {
    let user_id = env::var("USER_ID").expect("USER_ID must be set");
    let cursor = user::friends_ids(user_id.clone(), &token).with_page_size(page_size);
    return Ok(cursor.call().await?.response.ids.into_iter().collect());
}

async fn get_user_names(ids: &Vec<u64>, token: &Token) -> Result<Vec<String>, BoxError> {
    let mut result = vec![];

    for id in ids {
        let name = get_user_name(*id, &token).await?;
        result.push(name);
    }

    return Ok(result);
}

async fn get_user_name(id: u64, token: &Token) -> Result<String, BoxError> {
    return Ok(user::show(id, &token).await?.response.name);
}

fn get_list_id() -> ListID {
    let id = env::var("LIST_ID")
        .expect("LIST_ID must be set")
        .parse()
        .expect("LIST_ID must be a number");
    return ListID::from_id(id);
}

async fn create_list(token: &Token) -> Result<List, BoxError> {
    let list_name = "followees";
    let result = list::create(
        list_name.to_string(),
        true,
        Some(list_name.to_string()),
        &token,
    )
    .await?
    .response;

    info!("List created => {:?}", result);
    return Ok(result);
}

async fn get_list_members(
    list_id: ListID,
    token: &Token,
    page_size: i32,
) -> Result<HashSet<u64>, BoxError> {
    let cursor = list::members(list_id.clone(), &token).with_page_size(page_size);
    return Ok(cursor
        .call()
        .await?
        .response
        .users
        .into_iter()
        .map(|u| u.id)
        .collect());
}
