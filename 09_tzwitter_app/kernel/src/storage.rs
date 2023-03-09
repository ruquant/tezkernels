use crate::core::public_key_hash::PublicKeyHash;
use crate::core::tweet::Tweet;
use crate::core::{account::Account, error::*, nonce::Nonce};
use host::runtime::load_value_slice;
use host::{
    path::{concat, OwnedPath, RefPath},
    rollup_core::RawRollupCore,
    runtime::Runtime,
};

const ACCOUNTS: RefPath = RefPath::assert_from(b"/accounts");
pub const TWEETS: RefPath = RefPath::assert_from(b"/tweets");
const TWEET_COUNTER: RefPath = RefPath::assert_from(b"/constants/tweet-counter"); // The name constants is not appropriate

/// Compute the path /accounts/{tz1...}
fn account_path(public_key_hash: &PublicKeyHash) -> Result<OwnedPath> {
    let public_key_hash = public_key_hash.to_b58();
    let path: Vec<u8> = format!("/{}", public_key_hash).into();
    let path = OwnedPath::try_from(path).map_err(Error::from)?;
    concat(&ACCOUNTS, &path).map_err(Error::from)
}

/// Compute the path /accounts/{tz1...}/nonce
fn nonce_path(public_key_hash: &PublicKeyHash) -> Result<OwnedPath> {
    let nonce_path: Vec<u8> = "/nonce".into();
    let nonce_path = OwnedPath::try_from(nonce_path).map_err(Error::from)?;
    let account_path = account_path(public_key_hash)?;
    concat(&account_path, &nonce_path).map_err(Error::from)
}

/// Compute the path of the tweets /tweets/{hash}
fn tweet_path(tweet_id: &u64) -> Result<OwnedPath> {
    let tweet_path: Vec<u8> = format!("/{}", tweet_id).into();
    let tweet_path = OwnedPath::try_from(tweet_path).map_err(Error::from)?;
    concat(&TWEETS, &tweet_path).map_err(Error::from)
}

/// Compute the path of the tweet author
/// /tweets/{hash}/author
fn tweet_author_path(tweet_id: &u64) -> Result<OwnedPath> {
    let tweet_path = tweet_path(tweet_id)?;
    let tweet_author_path: Vec<u8> = "/author".into();
    let tweet_author_path = OwnedPath::try_from(tweet_author_path).map_err(Error::from)?;
    concat(&tweet_path, &tweet_author_path).map_err(Error::from)
}

/// Compute the path of the tweet content
/// /tweets/{hash}/content
fn tweet_content_path(tweet_id: &u64) -> Result<OwnedPath> {
    let tweet_path = tweet_path(tweet_id)?;
    let tweet_content_path: Vec<u8> = "/content".into();
    let tweet_content_path = OwnedPath::try_from(tweet_content_path).map_err(Error::from)?;
    concat(&tweet_path, &tweet_content_path).map_err(Error::from)
}

/// Read the nonce of a given account
///
/// Returns the default value of the Nonce if it does not exists
fn read_nonce<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    public_key_hash: &PublicKeyHash,
) -> Result<Nonce> {
    let path = nonce_path(public_key_hash)?;
    let is_present = Runtime::store_has(host, &path)?
        .map(|_| true)
        .unwrap_or_default();

    match is_present {
        false => Ok(Nonce::default()),
        true => {
            let mut buffer = [0_u8; 8];
            match load_value_slice(host, &path, &mut buffer) {
                Ok(8) => {
                    let nonce = u64::from_be_bytes(buffer);
                    Ok(Nonce(nonce))
                }
                _ => Err(Error::StateDeserializarion),
            }
        }
    }
}

/// Store the nonce of an account
///
/// Deletes the old value
fn store_nonce<'a, Host: RawRollupCore + Runtime>(
    host: &mut Host,
    public_key_hash: &PublicKeyHash,
    nonce: &'a Nonce,
) -> Result<&'a Nonce> {
    let path = nonce_path(public_key_hash)?;

    let data = nonce.0.to_be_bytes();
    let data = data.as_slice();

    host.store_write(&path, data, 0)
        .map_err(Error::from)
        .map(|_| nonce)
}

/// Store the author of a tweet
fn store_tweet_author<'a, Host: RawRollupCore + Runtime>(
    host: &mut Host,
    tweet_id: &u64,
    author: &'a PublicKeyHash,
) -> Result<&'a PublicKeyHash> {
    let path = tweet_author_path(tweet_id)?;
    let data = author.to_b58();
    let data = data.as_bytes();
    host.store_write(&path, data, 0)
        .map_err(Error::from)
        .map(|_| author)
}

/// store the tweet content
fn store_tweet_content<'a, Host: RawRollupCore + Runtime>(
    host: &mut Host,
    tweet_id: &u64,
    content: &'a str,
) -> Result<&'a str> {
    host.write_debug(content);

    let path = tweet_content_path(&tweet_id)?;
    let data = content.as_bytes();
    host.store_write(&path, data, 0)
        .map_err(Error::from)
        .map(|_| content)
}

/// Read the account of the user
pub fn read_account<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    public_key_hash: PublicKeyHash,
) -> Result<Account> {
    let nonce = read_nonce(host, &public_key_hash)?;
    Ok(Account {
        public_key_hash,
        nonce,
    })
}

/// Store an account to the location /account/{tz...}
///
/// Only the nonce is stored
pub fn store_account<'a, Host: RawRollupCore + Runtime>(
    host: &mut Host,
    account: &'a Account,
) -> Result<&'a Account> {
    let Account {
        nonce,
        public_key_hash,
    } = account;
    let _ = store_nonce(host, public_key_hash, nonce)?;
    Ok(account)
}

/// Store a tweet to the location /tweets/{tz...}
pub fn store_tweet<'a, Host: RawRollupCore + Runtime>(
    host: &mut Host,
    tweet_id: &u64,
    tweet: &'a Tweet,
) -> Result<&'a Tweet> {
    let Tweet { author, content } = tweet;
    let _ = store_tweet_author(host, tweet_id, author)?;
    let _ = store_tweet_content(host, tweet_id, content)?;
    Ok(tweet)
}

/// Increment the tweet counter and return the previous one.
pub fn increment_tweet_counter<Host: RawRollupCore + Runtime>(host: &mut Host) -> Result<u64> {
    let is_present = Runtime::store_has(host, &TWEET_COUNTER)?
        .map(|_| true)
        .unwrap_or_default();

    let previous_counter = match is_present {
        false => Ok(u64::default()),
        true => {
            let mut buffer = [0_u8; 8];
            match load_value_slice(host, &TWEET_COUNTER, &mut buffer) {
                Ok(8) => {
                    let id = u64::from_be_bytes(buffer);
                    Ok(id)
                }
                _ => Err(Error::StateDeserializarion),
            }
        }
    }?;

    let next_counter = previous_counter + 1;
    let data = next_counter.to_be_bytes();
    let data = data.as_slice();

    let _ = host
        .store_write(&TWEET_COUNTER, data, 0)
        .map_err(Error::from)?;

    Ok(previous_counter)
}
