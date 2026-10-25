mod gh_argument;

use std::env;

use gh_argument::GhArgument;

use crate::task::command_line::{CommandLine, Program};
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
        CommandLine::new(Program::GitHub)
            .argument(GhArgument::Api)
            .argument(GhArgument::Graphql)
            .argument(GhArgument::Field)
            .argument(format!("query={}", Self::QUERY))
            .argument(GhArgument::TypedField)
            .argument(format!("owner={owner}"))
            .argument(GhArgument::TypedField)
            .argument(format!("name={name}"))
            .argument(GhArgument::TypedField)
            .argument(format!("number={number}"))
            .argument(GhArgument::Jq)
            .argument(Self::BASE_AND_STACK)
            .stdout()
            .map(|answer| answer.trim().to_owned())
    }
}
