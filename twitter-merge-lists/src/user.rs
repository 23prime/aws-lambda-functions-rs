use std::hash::Hash;

use egg_mode::user::TwitterUser;

#[derive(Debug, Clone, Eq)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub screen_name: String,
}

impl User {
    pub fn from_twitter_user(twitter_user: TwitterUser) -> Self {
        return User {
            id: twitter_user.id,
            name: twitter_user.name,
            screen_name: twitter_user.screen_name,
        };
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

impl Hash for User {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
