//
// util.rs
// Copyright (C) 2022 Woshiluo Luo <woshiluo.luo@outlook.com>
// Distributed under terms of the GNU AGPLv3+ license.
//

use crate::CFToolError;

pub fn get_url(client: &reqwest::blocking::Client, url: &str) -> Result<String, CFToolError> {
    let builder = client.get(url);
    let respone = builder.send().map_err(|_| CFToolError::FailedRequest)?;
    if respone.status().is_success() {
        Ok(respone.text().map_err(|_| CFToolError::FailedRequest)?)
    } else {
        Err(crate::CFToolError::FailedRequest)
    }
}

pub fn gen_ftaa() -> String {
    use rand::{distributions::Alphanumeric, Rng};
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(18)
        .map(char::from)
        .collect()
}
