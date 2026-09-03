use std::fs;

use rustc_lexer::{FrontmatterAllowed, TokenKind, tokenize};

use crate::workspace::Workspace;

pub struct NoComments;

impl NoComments {
    pub fn check(workspace: &Workspace) -> Result<(), String> {
        let files = workspace.rust_files()?;
        let mut violations = Vec::new();
        for file in &files {
            let source =
                fs::read_to_string(file).map_err(|error| format!("{}: {error}", file.display()))?;
            let relative = workspace.relative(file);
            violations.extend(
                Self::comment_lines(&source)
                    .into_iter()
                    .map(|line| format!("{relative}:{line}")),
            );
        }
        if violations.is_empty() {
            println!("no comments found in {} rust files", files.len());
            Ok(())
        } else {
            Err(format!(
                "comments are not allowed in this repository:\n{}",
                violations.join("\n")
            ))
        }
    }

    fn comment_lines(source: &str) -> Vec<usize> {
        let mut lines = Vec::new();
        let mut offset = 0;
        for token in tokenize(source, FrontmatterAllowed::Yes) {
            if matches!(
                token.kind,
                TokenKind::LineComment { .. } | TokenKind::BlockComment { .. }
            ) {
                lines.push(source[..offset].matches('\n').count() + 1);
            }
            offset += token.len as usize;
        }
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::NoComments;

    #[test]
    fn flags_line_block_and_doc_comments() {
        let source = "fn a() {}\n// one\nfn b() {}\n/* two */\n/// three\nfn c() {}\n";
        assert_eq!(NoComments::comment_lines(source), vec![2, 4, 5]);
    }

    #[test]
    fn ignores_comment_syntax_inside_literals() {
        let source = "const URL: &str = \"https://example.com\";\nconst C: char = '/';\n";
        assert!(NoComments::comment_lines(source).is_empty());
    }
}
