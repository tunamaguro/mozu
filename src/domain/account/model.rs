use crate::domain::Id;
use std::sync::LazyLock;

pub type AccountId = Id<Account>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Account {
    id: AccountId,
    name: AccountName,
}

impl Account {
    pub fn new(id: Id<Account>, name: AccountName) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &Id<Account> {
        &self.id
    }
    pub fn name(&self) -> &AccountName {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccountName(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, thiserror::Error)]
pub enum AccountNameError {
    #[error("name {0} is invalid")]
    InvalidName(String),
}

impl AccountName {
    pub fn new(raw: &str) -> Result<Self, AccountNameError> {
        static REGEX: LazyLock<regex::Regex> =
            LazyLock::new(|| regex::Regex::new(r"^[a-zA-Z0-9_]{1,24}$").unwrap());

        if !REGEX.is_match(raw) {
            return Err(AccountNameError::InvalidName(raw.to_string()));
        }

        Ok(Self(raw.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub struct CreateAccountRequest {
    name: AccountName,
}

impl CreateAccountRequest {
    pub fn new(name: AccountName) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &AccountName {
        &self.name
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CreateAccountError {
    #[error("account name is invalid")]
    InvalidName(AccountNameError),
    #[error("account already exists")]
    AlreadyExists,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum FindAccountError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
