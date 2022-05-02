use crate::client::WebClient;
use crate::CFToolError;

impl WebClient {
    fn check_login(&mut self) -> Result<bool, CFToolError> {
        let body = self.get_url("https://codeforces.com/enter")?;

        let handle_regex = regex::Regex::new(r#"handle = "(.+?)""#).unwrap();
        let caps = handle_regex.captures(&body);

        match caps {
            Some(_) => Ok(true),
            _ => Ok(false),
        }
    }
    pub fn login(&mut self) -> Result<(), CFToolError> {
        if self.logined {
            return Ok(());
        }

        self.logined = true;

        if self.check_login()? {
            return Ok(());
        }

        let handle = self.config.handle.clone();
        let password = rpassword::prompt_password("Your password: ").unwrap();
        let _ = self.post_url(
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

        Ok(())
    }
}
