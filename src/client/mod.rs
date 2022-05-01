pub mod login;
pub mod parse;

use crate::CFToolError;
use regex::Regex;
use std::sync::Arc;

pub struct WebClient {
    client: reqwest::blocking::Client,
    cookies: Arc<reqwest::cookie::Jar>,
    has_rcpc: bool,
}

const PARMA_BFAA: &'static str = "f1b3f18c715565b589b7823cda7448ce";

impl WebClient {
    pub fn new() -> WebClient {
        let jar = Arc::from(reqwest::cookie::Jar::default());
        let client = reqwest::blocking::Client::builder()
            .cookie_store(true)
            .cookie_provider(Arc::clone(&jar))
            .build()
            .unwrap();
        WebClient {
            client,
            cookies: jar,
            has_rcpc: false,
        }
    }

    fn set_rcpc(&mut self) -> Result<(), CFToolError> {
        if self.has_rcpc {
            return Ok(());
        };
        use aes::cipher::{block_padding::ZeroPadding, BlockDecryptMut, KeyIvInit};

        let body = crate::util::get_url(&self.client, "https://codeforces.com")?;

        let number_regex = Regex::new(r#"toNumbers\("(.+?)"\)"#).unwrap();
        let caps = number_regex.captures_iter(&body);
        let caps: Vec<String> = caps.map(|cap| cap[1].to_string()).collect();

        let mut text: [u8; 16] = hex::decode(&caps[2]).unwrap().try_into().unwrap();
        let key: [u8; 16] = hex::decode(&caps[0]).unwrap().try_into().unwrap();
        let iv: [u8; 16] = hex::decode(&caps[1]).unwrap().try_into().unwrap();

        type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;
        let pt = Aes128CbcDec::new(&key.into(), &iv.into())
            .decrypt_padded_mut::<ZeroPadding>(&mut text)
            .unwrap();

        self.cookies.add_cookie_str(
            &format!("RCPC={}", hex::encode(pt)),
            &"https://codeforces.com".parse::<reqwest::Url>().unwrap(),
        );

        self.has_rcpc = true;

        Ok(())
    }

    fn get_csrf(&mut self, url: &str) -> Result<String, CFToolError> {
        let body = self.get_url(url)?;
        let csrf_regex = Regex::new(r#"csrf='(.+?)'"#).unwrap();
        let caps = csrf_regex.captures(&body).unwrap();

        Ok(caps[1].to_string())
    }

    pub fn get_url(&mut self, url: &str) -> Result<String, CFToolError> {
        self.set_rcpc()?;

        let builder = self.client.get(url);
        let respone = builder.send().map_err(|_| CFToolError::FailedRequest)?;

        if respone.status().is_success() {
            Ok(respone.text().map_err(|_| CFToolError::FailedRequest)?)
        } else {
            Err(crate::CFToolError::FailedRequest)
        }
    }

    pub fn post_url<'a>(
        &mut self,
        url: &str,
        csrf_url: &str,
        mut params: Vec<(&str, String)>,
    ) -> Result<String, CFToolError> {
        self.set_rcpc()?;

        let ftaa = crate::util::gen_ftaa();
        params.push(("bfaa", PARMA_BFAA.into()));
        params.push(("ftaa", ftaa));
        params.push(("csrf_token", self.get_csrf(csrf_url)?));

        let url = reqwest::Url::parse_with_params(url, params).unwrap();

        let builder = self.client.post(url);
        let respone = builder.send().map_err(|_| CFToolError::FailedRequest)?;

        if respone.status().is_success() {
            Ok(respone.text().map_err(|_| CFToolError::FailedRequest)?)
        } else {
            Err(crate::CFToolError::FailedRequest)
        }
    }
}
