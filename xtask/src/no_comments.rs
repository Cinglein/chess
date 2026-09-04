use rustc_lexer::{FrontmatterAllowed, TokenKind, tokenize};

use crate::source_file::SourceFile;

pub struct NoComments;

impl NoComments {
    pub fn check(files: &[SourceFile]) -> Result<(), String> {
        let violations: Vec<String> = files
            .iter()
            .flat_map(|file| {
                Self::comment_lines(&file.text)
                    .into_iter()
                    .map(move |line| format!("{}:{line}", file.path))
            })
            .collect();
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
        tokenize(source, FrontmatterAllowed::Yes)
            .scan(0, |offset, token| {
                let start = *offset;
                *offset += token.len as usize;
                Some((start, token.kind))
            })
            .filter(|(_, kind)| {
                matches!(
                    kind,
                    TokenKind::LineComment { .. } | TokenKind::BlockComment { .. }
                )
            })
            .map(|(start, _)| source[..start].matches('\n').count() + 1)
            .collect()
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
