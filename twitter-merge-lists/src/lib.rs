use log::info;
use std::collections::HashSet;
use std::env;

use dotenv::dotenv;
use egg_mode::list::ListID;
use egg_mode::{list, KeyPair, Token};

type BoxError = Box<dyn std::error::Error>;

pub mod error;
pub mod event;
pub mod logger;
pub mod user;

use user::User;

// See Twitter API Reference Standard v1.1:
// https://developer.twitter.com/en/docs/twitter-api/v1

pub async fn run() -> Result<(), BoxError> {
    dotenv().ok();
    const PAGE_SIZE: i32 = 10000;
    const EACH_API_CALL_LIMIT_SIZE: usize = 100;

    let token = get_token();

    let source_list_ids = get_source_list_ids();
    info!("Source lists: {:?}", source_list_ids);

    let source_list_members = get_source_list_members(&source_list_ids, &token, PAGE_SIZE).await?;
    info!(
        "Get {} source lists members: {:?}",
        source_list_members.len(),
        source_list_members
    );

    let target_list_id = get_target_list_id();
    info!("Target list: {:?}", target_list_id);

    let target_list_members = get_list_members(&target_list_id, &token, PAGE_SIZE).await?;
    info!(
        "Get {} target list members: {:?}",
        target_list_members.len(),
        target_list_members
    );

    let deficiency = (&source_list_members - &target_list_members)
        .into_iter()
        .collect::<Vec<_>>();
    info!("Get {} deficiency", deficiency.len());

    for partial_deficiency in split_vec(deficiency, EACH_API_CALL_LIMIT_SIZE) {
        let partial_deficiency_ids = partial_deficiency
            .clone()
            .into_iter()
            .map(|pd| pd.id)
            .collect::<Vec<_>>();
        let add_result = list::add_member_list(
            partial_deficiency_ids.clone(),
            target_list_id.clone(),
            &token,
        )
        .await;
        info!(
            "Adding result: {:?} => {:?}",
            partial_deficiency, add_result
        );
    }

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

fn get_source_list_ids() -> Vec<ListID> {
    let ids = env::var("SOURCE_LISTS").expect("SOURCE_LISTS must be set");
    return ids.split(',').into_iter().map(get_list_id).collect();
}

fn get_target_list_id() -> ListID {
    let id = env::var("TARGET_LIST").expect("TARGET_LIST must be set");
    return get_list_id(&id);
}

fn get_list_id(id: &str) -> ListID {
    return ListID::from_id(id.parse().expect("id must be a number"));
}

async fn get_source_list_members(
    list_ids: &[ListID],
    token: &Token,
    page_size: i32,
) -> Result<HashSet<User>, BoxError> {
    let mut result = HashSet::new();

    for list_id in list_ids {
        let list_members = get_list_members(list_id, token, page_size).await?;
        result.extend(list_members);
    }

    return Ok(result);
}

async fn get_list_members(
    list_id: &ListID,
    token: &Token,
    page_size: i32,
) -> Result<HashSet<User>, BoxError> {
    let cursor = list::members(list_id.clone(), token).with_page_size(page_size);
    let twitter_users = cursor.call().await?.response.users;
    return Ok(twitter_users
        .into_iter()
        .map(User::from_twitter_user)
        .collect());
}
