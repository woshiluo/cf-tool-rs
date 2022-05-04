use crate::CFToolError;

pub fn get_url(url: &str) -> Result<String, CFToolError> {
    let client = reqwest::blocking::Client::builder().build().unwrap();
    let builder = client.get(url);
    let respone = builder.send().map_err(|_| CFToolError::FailedRequest)?;
    if respone.status().is_success() {
        Ok(respone.text().map_err(|_| CFToolError::FailedRequest)?)
    } else {
        Err(crate::CFToolError::FailedRequest)
    }
}

pub fn write_sample(
    samples: (Vec<String>, Vec<String>),
    problem: &str,
    base: impl AsRef<std::path::Path>,
) {
    let base = base.as_ref();
    let (inputs, outputs) = samples;
    std::fs::create_dir_all(&base).unwrap();

    let size = inputs.len();
    for i in 1..=size {
        use std::io::Write;
        {
            let mut file =
                std::fs::File::create(format!("{}{}{}.in", base.to_str().unwrap(), &problem, i))
                    .unwrap();
            file.write_all(inputs[i - 1].as_ref()).unwrap();
        }
        {
            let mut file =
                std::fs::File::create(format!("{}{}{}.ans", base.to_str().unwrap(), &problem, i))
                    .unwrap();
            file.write_all(outputs[i - 1].as_ref()).unwrap();
        }
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
