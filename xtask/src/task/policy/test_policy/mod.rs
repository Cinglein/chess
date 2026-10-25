use serde::{Deserialize, Serialize};

mod percent;

use percent::Percent;

use super::most::Most;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestPolicy {
    tests_per_file: Most,
    assertions_per_test: Most,
    lines_per_test: Most,
    literals_per_test: Most,
    largest_integer: Most,
    tests_per_hundred_files: Percent,
    test_line_percent: Percent,
}

impl TestPolicy {
    pub const fn tests_per_file(&self) -> Most {
        self.tests_per_file
    }

    pub const fn assertions_per_test(&self) -> Most {
        self.assertions_per_test
    }

    pub const fn lines_per_test(&self) -> Most {
        self.lines_per_test
    }

    pub const fn literals_per_test(&self) -> Most {
        self.literals_per_test
    }

    pub const fn largest_integer(&self) -> Most {
        self.largest_integer
    }

    pub const fn tests_per_hundred_files(&self) -> Percent {
        self.tests_per_hundred_files
    }

    pub const fn test_line_percent(&self) -> Percent {
        self.test_line_percent
    }
}
