//
// login.rs
// Copyright (C) 2022 Woshiluo Luo <woshiluo.luo@outlook.com>
// Distributed under terms of the GNU AGPLv3+ license.
//

use crate::client::WebClient;
use crate::CFToolError;

impl WebClient {
    pub fn login(&mut self, handle: &str, password: &str) -> Result<(), CFToolError> {
        let body = self.post_url(
            "https://codeforces.com/enter",
            "https://codeforces.com/enter",
            vec![
                ("handleOrEmail", handle.into()),
                ("password", password.into()),
                ("action", "enter".into()),
                ("_tta", "176".into()),
                ("remember", "on".into()),
            ],
        )?;
        println!("{}", body);
        Ok(())
    }
}
