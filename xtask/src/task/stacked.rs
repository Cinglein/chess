use std::env;
use std::process::Command;

use crate::task::failure::Failure;

pub struct Stacked;

impl Stacked {
    const NUMBER: &str = "PULL_REQUEST";
    const REPOSITORY: &str = "GITHUB_REPOSITORY";
    const DEFAULT_BRANCH: &str = "main";
    const QUERY: &str = "query($owner: String!, $name: String!, $number: Int!) { repository(owner: $owner, name: $name) { pullRequest(number: $number) { baseRefName stack { number } } } }";
    const BASE_AND_STACK: &str =
        r#".data.repository.pullRequest | "\(.baseRefName) \(.stack.number // "none")""#;
    const NONE: &str = "none";

    pub fn run() -> Result<(), Failure> {
        let number = Self::required(Self::NUMBER)?;
        let repository = Self::required(Self::REPOSITORY)?;
        let (owner, name) = repository
            .split_once('/')
            .ok_or_else(|| Failure::Usage(format!("{} must be owner/name", Self::REPOSITORY)))?;
        let answer = Self::base_and_stack(owner, name, &number)?;
        let (base, stack) = answer
            .split_once(' ')
            .ok_or_else(|| Failure::GitHub(answer.clone()))?;
        if base == Self::DEFAULT_BRANCH {
            println!("stacked: pull request #{number} targets {base}");
            return Ok(());
        }
        if stack == Self::NONE {
            return Err(Failure::Refused(format!(
                "pull request #{number} targets {base} but belongs to no GitHub stack; link it with gh stack link <bottom> ... <top>"
            )));
        }
        println!("stacked: pull request #{number} targets {base} inside stack #{stack}");
        Ok(())
    }

    fn required(name: &str) -> Result<String, Failure> {
        env::var(name).map_err(|_| Failure::Usage(format!("{name} must be set")))
    }

    fn base_and_stack(owner: &str, name: &str, number: &str) -> Result<String, Failure> {
        let output = Command::new("gh")
            .args([
                "api",
                "graphql",
                "-f",
                &format!("query={}", Self::QUERY),
                "-F",
                &format!("owner={owner}"),
                "-F",
                &format!("name={name}"),
                "-F",
                &format!("number={number}"),
                "--jq",
                Self::BASE_AND_STACK,
            ])
            .output()
            .map_err(|error| Failure::GitHub(error.to_string()))?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
        } else {
            Err(Failure::GitHub(
                String::from_utf8_lossy(&output.stderr).trim().to_owned(),
            ))
        }
    }
}
