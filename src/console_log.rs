/// This is the output a ConsoleLogParser::parse_line()
/// Only the output of the status command are handled.
#[derive(Debug, PartialEq)]
pub enum LogLine {
    Unknown,
    PlayerInfo {
        steam_id: String,
        name: String,
        id: u32,
    },
}

pub trait ConsoleLogParser {
    fn parse_line(&self, text: &str) -> LogLine;
}
