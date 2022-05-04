use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use scraper::{Html, Selector};

use crate::client::WebClient;
use crate::CFToolError;

impl WebClient {
    fn contest_started(&mut self, contest_id: u32) -> Result<Option<u64>, CFToolError> {
        let body = self.get_url(&format!(
            "https://codeforces.com/contest/{}/countdown",
            contest_id
        ))?;
        Ok(match body.find("Before the contest") {
            Some(_) => {
                let time_regex = regex::Regex::new(r#"(\d+):(\d+):(\d+)"#).unwrap();
                let caps = time_regex.captures(&body).unwrap();

                Some(
                    caps[1].parse::<u64>().unwrap() * 60 * 60
                        + caps[2].parse::<u64>().unwrap() * 60
                        + caps[3].parse::<u64>().unwrap(),
                )
            }
            _ => None,
        })
    }

    pub fn race(&mut self, contest_id: u32) -> Result<(), CFToolError> {
        if !self.logined {
            return Err(CFToolError::NotLogin);
        }

        println!("Race {}", contest_id);
        if let Some(mut time) = self.contest_started(contest_id)? {
            loop {
                let mut stdout = stdout();
                let wait_time =
                    std::time::Instant::now() + Duration::from_secs(std::cmp::min(time, 900));

                while std::time::Instant::now() < wait_time {
                    execute!(
                        stdout,
                        Clear(ClearType::CurrentLine),
                        cursor::MoveToColumn(1)
                    )
                    .map_err(|_| CFToolError::FailedTerminalOutput)?;

                    let seconds = time % 60;
                    let minutes = time / 60 % 60;
                    let hours = time / 60 / 60;

                    print!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds);
                    stdout
                        .flush()
                        .map_err(|_| CFToolError::FailedTerminalOutput)?;

                    sleep(Duration::from_secs(1));
                }

                match self.contest_started(contest_id)? {
                    Some(new_time) => time = new_time,
                    _ => break,
                }
            }
        };

        let body = self.get_url(&format!("https://codeforces.com/contest/{}", contest_id))?;
        let mut problems: Vec<String> = vec![];

        let fragment = Html::parse_fragment(&body);
        let tr_selector = Selector::parse(".problems > tbody > tr").unwrap();

        for tr in fragment.select(&tr_selector) {
            let problem_selector = Selector::parse("td > a").unwrap();
            let problem = tr.select(&problem_selector).next();
            if let Some(problem) = problem {
                problems.push(problem.inner_html().to_string().trim().to_lowercase());
            }
        }

        if problems.is_empty() {
            return Err(CFToolError::FailedParseRespone);
        }

        for problem in problems {
            crate::util::write_sample(
                self.parse(contest_id, &problem)?,
                &problem,
                format!("./{}/{}/", contest_id, problem),
            );
        }

        Ok(())
    }
}
