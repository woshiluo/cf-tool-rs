# cf-tool-rs

A Rust implement for <https://github.com/xalanq/cf-tool>

WIP. Pull Requests / Contributions are welcomed!

## How to Configure?

Configure File should in `~/.config/cf-tool-rs/config.toml`: 

```toml
session_file="/tmp/cf-tool.session"
handle="woshiluo"
code_suffix=".cpp"
language_id="54"
```

- `session_file`: Save cookies in this file.
- `handle`: Your codeforces handle
- `code_suffix`: Source code filename suffix
- `language_id`: The language's id which you want to submit. (You can check your language's id in this file <https://github.com/woshiluo/cf-tool/blob/master/client/langs.go>)

## Usage

Three Subcommands

- `race <CID>`: If the contest <CID> has not started yet, program will count down. If it has started, the program will parse this contest's problem in `./<CID>/<PID>`.
- `parse <CID> <PID>`: Parse this problem's sample-cases and write in current dir.
- `submit`: Your current dir should be like `<CID>/<PID>`, the program will submit `<PID>.<code_suffix>`.

## TO-DO

- [x] login
- [x] parse
- [x] race
  - [x] public contest
  - [x] login
  - [x] Countdown
- [x] submit
- [x] config file
- [x] session file
- [x] Colorful output

## Interested 

- [ ] watch
- [ ] test
- [ ] list

## Not Interested

- Template
