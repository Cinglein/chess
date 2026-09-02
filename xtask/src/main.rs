use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use rustc_lexer::{FrontmatterAllowed, TokenKind, tokenize};

fn main() -> ExitCode {
    let task = env::args().nth(1);
    let result = match task.as_deref() {
        Some("ci") => ci(),
        Some("no-comments") => no_comments(),
        _ => {
            eprintln!("usage: cargo xtask <ci|no-comments>");
            return ExitCode::FAILURE;
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lives directly under the workspace root")
        .to_path_buf()
}

fn ci() -> Result<(), String> {
    cargo(&["fmt", "--all", "--check"])?;
    cargo(&[
        "clippy",
        "--workspace",
        "--all-targets",
        "--all-features",
        "--",
        "-D",
        "warnings",
    ])?;
    cargo(&["test", "--workspace", "--all-features"])?;
    no_comments()
}

fn cargo(args: &[&str]) -> Result<(), String> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let status = Command::new(cargo)
        .args(args)
        .current_dir(workspace_root())
        .status()
        .map_err(|error| format!("failed to run cargo {}: {error}", args.join(" ")))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("cargo {} failed", args.join(" ")))
    }
}

fn no_comments() -> Result<(), String> {
    let root = workspace_root();
    let mut files = Vec::new();
    collect_rust_files(&root, &mut files)?;
    files.sort();
    let mut violations = Vec::new();
    for file in &files {
        let source =
            fs::read_to_string(file).map_err(|error| format!("{}: {error}", file.display()))?;
        let relative = file.strip_prefix(&root).unwrap_or(file).display();
        for line in comment_lines(&source) {
            violations.push(format!("{relative}:{line}"));
        }
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

fn collect_rust_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = fs::read_dir(dir).map_err(|error| format!("{}: {error}", dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("{}: {error}", dir.display()))?;
        let path = entry.path();
        if path.is_dir() {
            let name = entry.file_name();
            if name != "target" && name != ".git" {
                collect_rust_files(&path, out)?;
            }
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            out.push(path);
        }
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::comment_lines;

    #[test]
    fn flags_line_block_and_doc_comments() {
        let source = "fn a() {}\n// one\nfn b() {}\n/* two */\n/// three\nfn c() {}\n";
        assert_eq!(comment_lines(source), vec![2, 4, 5]);
    }

    #[test]
    fn ignores_comment_syntax_inside_literals() {
        let source = "const URL: &str = \"https://example.com\";\nconst C: char = '/';\n";
        assert!(comment_lines(source).is_empty());
    }
}
