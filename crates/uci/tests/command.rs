use uci::{Command, Urgency};
const LINES: [&str; 11] = [
    "uci",
    "isready",
    "setoption name UCI_Elo value 1320",
    "ucinewgame",
    "position startpos",
    "position startpos moves e2e4 e7e5",
    "position fen 8/8/8/8/8/8/8/K6k w - - 0 1 moves a1a2",
    "go infinite",
    "go wtime 1 btime 2 winc 3 binc 4",
    "stop",
    "quit",
];

#[test]
fn every_command_prints_back_to_the_line_it_was_parsed_from_and_unknown_words_are_rejected() {
    for line in LINES {
        let command = Command::try_from(line).unwrap();
        assert_eq!(command.to_string(), line);
        let interrupting = command.urgency() == Urgency::Interrupting;
        assert_eq!(interrupting, line == "stop" || line == "quit");
    }
    assert!(Command::try_from("dance").is_err());
}
