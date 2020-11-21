use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use crate::console_log::{ConsoleLogParser, LogLine};
use crate::console_log_parser_line_based::ConsoleLogParserLineBased;
use crate::utils::BoxResult;

#[derive(Debug)]
pub struct LogFileWatcher {
    pub filename: String,
    pub last_pos: u64,
    pub parser: ConsoleLogParserLineBased,
}

impl LogFileWatcher {
    pub fn new<S: Into<String>>(filename: S, parser: ConsoleLogParserLineBased) -> Self {
        let filename = filename.into();
        let file = File::open(&filename).unwrap();
        let last_pos = file.metadata().unwrap().len();

        LogFileWatcher {
            filename,
            last_pos,
            parser,
        }
    }

    pub fn process_new_data(&mut self) -> Vec<LogLine> {
        let mut infos = Vec::new();

        let new_data = self.read_new_data();
        if let Ok(new_data) = new_data {
            for line in new_data.lines() {
                let info = self.parser.parse_line(line);
                infos.push(info);
            }
        } else {
            println!(
                "LogFileWatcher.process_new_data: No new data! Error: {:#?}",
                new_data
            );
        }

        infos
    }

    fn read_new_data(&mut self) -> BoxResult<String> {
        let mut file = File::open(self.filename.as_str()).unwrap();

        // Get new file length, if same as old, we're done.
        let new_pos = file.metadata()?.len();
        if new_pos == self.last_pos {
            return Ok("".to_string());
        }

        // Seek to last pos
        file.seek(SeekFrom::Start(self.last_pos))?;

        // Read the portion of the file that is new
        let mut len = (new_pos - self.last_pos) as usize;
        let mut buf: Vec<u8> = Vec::with_capacity(len);
        buf.resize(len, 0);
        file.read_exact(&mut buf)?;

        // buf now contain the new data as a Vec<u8>

        // TODO: Adjust size of buf so it ends with a newline

        len = buf.len();

        let s = String::from_utf8(buf)?;

        self.last_pos += len as u64;

        Ok(s)
    }
}
