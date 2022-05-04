pub mod login;
pub mod parse;
pub mod race;
pub mod submit;

use crate::CFToolError;
use regex::Regex;
use std::sync::Arc;

pub struct WebClient {
    client: reqwest::blocking::Client,
    config: crate::config::Config,
    cookies: Arc<reqwest_cookie_store::CookieStoreMutex>,
    has_rcpc: bool,
    logined: bool,
}

impl Drop for WebClient {
    fn drop(&mut self) {
        let mut file = std::fs::File::create(&self.config.session_file)
            .map(std::io::BufWriter::new)
            .unwrap();
        let cookies = self.cookies.lock().unwrap();
        cookies.save_json(&mut file).unwrap();
    }
}

const PARMA_BFAA: &str = "f1b3f18c715565b589b7823cda7448ce";

impl WebClient {
    pub fn new(config: crate::config::Config) -> WebClient {
        let cookie_store = {
            let file = std::fs::File::open(&config.session_file).map(std::io::BufReader::new);
            match file {
                Ok(file) => cookie_store::CookieStore::load_json(file).unwrap(),
                _ => cookie_store::CookieStore::default(),
            }
        };
        let jar = Arc::from(reqwest_cookie_store::CookieStoreMutex::new(cookie_store));

        let client = reqwest::blocking::Client::builder()
            .cookie_store(true)
            .cookie_provider(Arc::clone(&jar))
            .build()
            .unwrap();

        WebClient {
            client,
            config,
            cookies: jar,
            has_rcpc: false,
            logined: false,
        }
    }

    fn set_rcpc(&mut self) -> Result<(), CFToolError> {
        if self.has_rcpc {
            return Ok(());
        };

        self.has_rcpc = true;

        use aes::cipher::{block_padding::ZeroPadding, BlockDecryptMut, KeyIvInit};

        let body = crate::util::get_url("https://codeforces.com")?;

        // There is no rcpc.
        if !body.contains("Redirecting") {
            return Ok(());
        }

        // User Regex to get aes triple
        let number_regex = Regex::new(r#"toNumbers\("(.+?)"\)"#).unwrap();
        let caps = number_regex.captures_iter(&body);
        let caps: Vec<String> = caps.map(|cap| cap[1].to_string()).collect();

        let mut text: [u8; 16] = hex::decode(&caps[2])
            .map_err(|_| CFToolError::FailedParseRespone)?
            .try_into()
            .map_err(|_| CFToolError::FailedParseRespone)?;
        let key: [u8; 16] = hex::decode(&caps[0])
            .map_err(|_| CFToolError::FailedParseRespone)?
            .try_into()
            .map_err(|_| CFToolError::FailedParseRespone)?;
        let iv: [u8; 16] = hex::decode(&caps[1])
            .map_err(|_| CFToolError::FailedParseRespone)?
            .try_into()
            .map_err(|_| CFToolError::FailedParseRespone)?;

        // Decrypt
        type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;
        let pt = Aes128CbcDec::new(&key.into(), &iv.into())
            .decrypt_padded_mut::<ZeroPadding>(&mut text)
            .map_err(|_| CFToolError::FailedParseRespone)?;

        // Set rcpc
        {
            let mut cookies = self.cookies.lock().unwrap();
            cookies
                .parse(
                    &format!("RCPC={}", hex::encode(pt)),
                    &"https://codeforces.com".parse::<reqwest::Url>().unwrap(),
                )
                .map_err(|_| CFToolError::FailedRequest)?;
        }

        Ok(())
    }

    fn get_csrf(&mut self, url: &str) -> Result<String, CFToolError> {
        let body = self.get_url(url)?;
        let csrf_regex = Regex::new(r#"csrf='(.+?)'"#).unwrap();
        let caps = csrf_regex
            .captures(&body)
            .ok_or(CFToolError::FailedParseRespone)?;

        Ok(caps[1].to_string())
    }

    // TODO: Return status code?
    // TODO: Block redir?
    pub fn get_url(&mut self, url: &str) -> Result<String, CFToolError> {
        self.set_rcpc()?;

        let builder = self.client.get(url);
        let respone = builder.send().map_err(|_| CFToolError::FailedRequest)?;

        if respone.status().is_success() {
            Ok(respone.text().map_err(|_| CFToolError::FailedRequest)?)
        } else {
            Err(crate::CFToolError::WrongRespone(respone.status().as_u16()))
        }
    }

    pub fn post_url(
        &mut self,
        url: &str,
        csrf_url: &str,
        mut params: Vec<(&str, String)>,
    ) -> Result<String, CFToolError> {
        self.set_rcpc()?;

        // Construct parmas
        let ftaa = crate::util::gen_ftaa();
        params.push(("bfaa", PARMA_BFAA.into()));
        params.push(("ftaa", ftaa));
        params.push(("csrf_token", self.get_csrf(csrf_url)?));
        let url =
            reqwest::Url::parse_with_params(url, params).map_err(|_| CFToolError::FailedRequest)?;

        let builder = self.client.post(url);
        let respone = builder.send().map_err(|_| CFToolError::FailedRequest)?;

        if respone.status().is_success() {
            Ok(respone.text().map_err(|_| CFToolError::FailedRequest)?)
        } else {
            Err(crate::CFToolError::WrongRespone(respone.status().as_u16()))
        }
    }
}
