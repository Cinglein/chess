use game::{ArenaError, Opponent};
use opponent_ports::UciProcess;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

const SCRATCH: &str = env!("CARGO_TARGET_TMPDIR");
const EXECUTABLE: u32 = 0o755;
const READY_BEFORE_UCIOK: &str = "read line; printf 'readyok\nreadyok\n'";
const GONE_BEFORE_READYOK: &str = "read line; printf 'uciok\n'";
const GONE_AT_NEW_GAME: &str = "while read line; do case $line in \
    uci) echo uciok;; isready) echo readyok;; ucinewgame) exit;; esac; done";

struct ScriptedEngine;

impl ScriptedEngine {
    fn written(name: &str, script: &str) -> PathBuf {
        let path = Path::new(SCRATCH).join(name);
        fs::write(&path, format!("#!/bin/sh\n{script}\n")).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(EXECUTABLE)).unwrap();
        path
    }

    fn spawned(name: &str, script: &str) -> Result<UciProcess, ArenaError> {
        UciProcess::spawn(&Self::written(name, script), &[])
    }
}

#[test]
fn an_engine_that_skips_a_handshake_step_is_refused_at_that_step() {
    assert!(ScriptedEngine::spawned("no-uciok", READY_BEFORE_UCIOK).is_err());
    assert!(ScriptedEngine::spawned("no-readyok", GONE_BEFORE_READYOK).is_err());
    let mut engine = ScriptedEngine::spawned("no-new-game", GONE_AT_NEW_GAME).unwrap();
    assert!(engine.begin_game().is_err());
}
