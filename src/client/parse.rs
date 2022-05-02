use crate::client::WebClient;
use crate::CFToolError;

impl WebClient {
    pub fn parse(
        &mut self,
        contest_id: u32,
        problem_id: &str,
    ) -> Result<(Vec<String>, Vec<String>), CFToolError> {
        let problem_id = problem_id.to_lowercase();
        if !self.logined {
            return Err(CFToolError::NotLogin);
        }
        println!("Parsing {} {}", contest_id, &problem_id);
        let body = self.get_url(&format!(
            "https://codeforces.com/contest/{}/problem/{}",
            contest_id, problem_id,
        ))?;

        use scraper::{Html, Selector};
        let fragment = Html::parse_fragment(&body);
        let input_selector = Selector::parse(".sample-test > .input > pre").unwrap();
        let output_selector = Selector::parse(".sample-test > .output > pre").unwrap();

        let mut input_cases = vec![];
        let mut output_cases = vec![];

        let inputs = fragment.select(&input_selector);
        for input in inputs {
            input_cases.push(input.inner_html().to_string());
        }

        let outputs = fragment.select(&output_selector);
        for output in outputs {
            output_cases.push(output.inner_html().to_string());
        }

        Ok((input_cases, output_cases))
    }
}
