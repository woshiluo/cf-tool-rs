use std::fs::File;
use std::io::Read;

use crate::client::WebClient;
use crate::CFToolError;

impl WebClient {
    fn gen_code_filename(&self, problem_id: &str) -> Result<String, CFToolError> {
        Ok(format!("{}{}", problem_id, self.config.code_suffix))
    }
    pub fn submit(&mut self, contest_id: u32, problem_id: &str) -> Result<(), CFToolError> {
        println!("Submitting {} {}", contest_id, problem_id);

        let mut file = File::open(self.gen_code_filename(problem_id)?).unwrap();
        let mut source_code = String::new();

        file.read_to_string(&mut source_code).unwrap();

        let submit_url = format!("https://codeforces.com/contest/{}/submit", contest_id);
        let body = self.post_url(
            &submit_url,
            &submit_url,
            vec![
                ("action", "submitSolutionFormSubmitted".into()),
                ("submittedProblemIndex", problem_id.into()),
                ("programTypeId", self.config.language_id.to_string()),
                ("contestId", contest_id.to_string()),
                ("source", source_code.into()),
                ("tabSize", "4".into()),
                ("_tta", "594".into()),
                ("sourceCodeConfirmed", "true".into()),
            ],
        )?;

        let error_regex = regex::Regex::new(r#"error[a-zA-Z_\- ]*">(.*?)</span>"#).unwrap();
        let error_caps = error_regex.captures(&body);

        if error_caps.is_some() {
            println!("Submit Failed: {}", error_caps.unwrap()[1].to_string());
            return Err(CFToolError::FailedRequest);
        }

        println!("Submitted");
        Ok(())
    }
}
