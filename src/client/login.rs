use crate::client::WebClient;
use crate::CFToolError;

use crossterm::{
    execute,
    style::{Color, ResetColor, SetForegroundColor},
};

impl WebClient {
    fn check_login(&mut self) -> Result<Option<String>, CFToolError> {
        let body = self.get_url("https://codeforces.com/enter")?;

        let handle_regex = regex::Regex::new(r#"handle = "(.+?)""#).unwrap();
        let caps = handle_regex.captures(&body);

        Ok(match caps {
            Some(caps) => Some(caps[1].to_string()),
            _ => None,
        })
    }
    pub fn login(&mut self) -> Result<(), CFToolError> {
        let mut stdout = std::io::stdout();
        if self.logined {
            return Ok(());
        }

        self.logined = true;

        if let Some(handle) = self.check_login()? {
            execute!(stdout, SetForegroundColor(Color::Green))?;
            println!("Current user: {}", handle);
            execute!(stdout, ResetColor)?;
            return Ok(());
        }

        execute!(stdout, SetForegroundColor(Color::Red))?;
        println!("Not logged in, Try login {}", self.config.handle);
        execute!(stdout, ResetColor)?;

        let handle = self.config.handle.clone();
        let password = rpassword::prompt_password("Your password: ").unwrap();
        let _ = self.post_url(
            "https://codeforces.com/enter",
            "https://codeforces.com/enter",
            vec![
                ("handleOrEmail", handle),
                ("password", password),
                ("action", "enter".into()),
                ("_tta", "176".into()),
                ("remember", "on".into()),
            ],
        )?;

        execute!(stdout, SetForegroundColor(Color::Green))?;
        println!("Logged in");
        execute!(stdout, ResetColor)?;

        Ok(())
    }
}
