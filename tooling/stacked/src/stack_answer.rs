use std::str::FromStr;

use task::Failure;

pub struct StackAnswer {
    base: String,
    stack: Option<String>,
}

impl StackAnswer {
    const DEFAULT_BRANCH: &str = "main";
    const NONE: &str = "none";

    pub fn verdict(&self, number: &str) -> Result<String, Failure> {
        match (&self.base, &self.stack) {
            (base, _) if base == Self::DEFAULT_BRANCH => {
                Ok(format!("stacked: pull request #{number} targets {base}"))
            }
            (base, Some(stack)) => Ok(format!(
                "stacked: pull request #{number} targets {base} inside stack #{stack}"
            )),
            (base, None) => Err(Failure::Refused(format!(
                "pull request #{number} targets {base} but belongs to no GitHub stack; link it with gh stack link <bottom> ... <top>"
            ))),
        }
    }
}

impl FromStr for StackAnswer {
    type Err = Failure;

    fn from_str(text: &str) -> Result<StackAnswer, Failure> {
        let (base, stack) = text
            .trim()
            .split_once(' ')
            .ok_or_else(|| Failure::GitHub(text.to_owned()))?;
        Ok(StackAnswer {
            base: base.to_owned(),
            stack: (stack != Self::NONE).then(|| stack.to_owned()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::StackAnswer;
    const ANSWERS: [&str; 3] = ["main none", "feature 12", "feature none"];
    #[test]
    fn a_pull_request_passes_on_main_or_inside_a_stack_and_is_refused_loose_off_main() {
        let verdict = |answer: &str| answer.parse::<StackAnswer>().unwrap().verdict("7");
        let [on_main, stacked, loose] = ANSWERS.map(verdict);
        let fine = on_main.is_ok() && stacked.unwrap().contains("stack #12") && loose.is_err();
        assert!(fine && "garbage".parse::<StackAnswer>().is_err());
    }
}
