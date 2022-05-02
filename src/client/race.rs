use crate::client::WebClient;
use crate::CFToolError;

impl WebClient {
    pub fn race(&mut self, contest_id: u32) -> Result<(), CFToolError> {
        // TODO: CountDown
        if !self.logined {
            return Err(CFToolError::NotLogin);
        }

        let body = self.get_url(&format!("https://codeforces.com/contest/{}", contest_id))?;
        let mut problems: Vec<String> = vec![];

        use scraper::{Html, Selector};
        let fragment = Html::parse_fragment(&body);
        let tr_selector = Selector::parse(".problems > tbody > tr").unwrap();

        for tr in fragment.select(&tr_selector) {
            let problem_selector = Selector::parse("td > a").unwrap();
            let problem = tr.select(&problem_selector).next();
            if let Some(problem) = problem {
                problems.push(problem.inner_html().to_string().trim().to_lowercase());
            }
        }

        if problems.len() == 0 {
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
