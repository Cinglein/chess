mod gh_argument;
mod gh_field;
mod gh_key;
mod repository;
mod stack_answer;

use std::env;

use gh_argument::GhArgument;
use gh_field::GhField;
use gh_key::GhKey;
use repository::Repository;
use stack_answer::StackAnswer;

use task::Failure;
use task::{CommandLine, Program, Stdout};

pub struct Stacked;

impl Stacked {
    const NUMBER: &str = "PULL_REQUEST";
    const REPOSITORY: &str = "GITHUB_REPOSITORY";
    const QUERY: &str = "query($owner: String!, $name: String!, $number: Int!) { repository(owner: $owner, name: $name) { pullRequest(number: $number) { baseRefName stack { number } } } }";
    const BASE_AND_STACK: &str =
        r#".data.repository.pullRequest | "\(.baseRefName) \(.stack.number // "none")""#;

    pub fn run() -> Result<(), Failure> {
        let number = Self::required(Self::NUMBER)?;
        let repository: Repository = Self::required(Self::REPOSITORY)?.parse()?;
        let answer: StackAnswer = Self::base_and_stack(&repository, &number)?.parse()?;
        println!("{}", answer.verdict(&number)?);
        Ok(())
    }

    fn required(name: &str) -> Result<String, Failure> {
        env::var(name).map_err(|_| Failure::Usage(format!("{name} must be set")))
    }

    fn base_and_stack(repository: &Repository, number: &str) -> Result<String, Failure> {
        let [owner, name] = repository.fields();
        CommandLine::new(Program::GitHub)
            .argument(GhArgument::Api)
            .argument(GhArgument::Graphql)
            .argument(GhArgument::Field)
            .argument(GhField::new(GhKey::Query, Self::QUERY))
            .argument(GhArgument::TypedField)
            .argument(owner)
            .argument(GhArgument::TypedField)
            .argument(name)
            .argument(GhArgument::TypedField)
            .argument(GhField::new(GhKey::Number, number))
            .argument(GhArgument::Jq)
            .argument(Self::BASE_AND_STACK)
            .captured::<Stdout>()
    }
}
