use regex::Regex;

/// This is the output a ConsoleLogParser::parse_line()
#[derive(Debug, PartialEq)]
pub enum LogLineInfo {
    Nothing,
    PlayerInfo {
        steam_id: String,
        name: String,
        id: u32,
    },
}

#[derive(Debug)]
pub struct ConsoleLogParser {
    player_info_regex: Regex,
}

const REGEX_TIMESTAMP_STR: &'static str = r"\d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}";

impl ConsoleLogParser {
    pub fn new() -> Self {
        ConsoleLogParser {
            player_info_regex: Self::player_info_regex(),
        }
    }

    pub fn player_info_regex() -> Regex {
        let player_info_regex = format!(
            r#"^({}): #\s+(\d+)\s+"(.*)"\s+\[(U:\d:\d+)\]\s+.*$"#,
            REGEX_TIMESTAMP_STR
        );
        Regex::new(player_info_regex.as_str()).unwrap()
    }

    pub fn parse_line(&self, line: &str) -> LogLineInfo {
        if let Some(player_info) = self.player_info_regex.captures(line) {
            LogLineInfo::PlayerInfo {
                steam_id: player_info[4].to_string(),
                name: player_info[3].to_string(),
                id: player_info[2].parse::<u32>().unwrap_or_default(),
            }
        } else {
            LogLineInfo::Nothing
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_console_parse_player_info_line() {
        let parser = ConsoleLogParser::new();

        let line = r#"11/07/2020 - 08:41:39: #     85 "aftershave"        [U:1:13962573]      01:44       44    0 active"#;

        let info = parser.parse_line(line);
        println!("{:#?}", info);
        assert_eq!(
            info,
            LogLineInfo::PlayerInfo {
                id: 85,
                steam_id: "U:1:13962573".to_string(),
                name: r#"aftershave"#.to_string()
            }
        );
    }

    #[test]
    fn test_console_parse_player_info_tricky_1() {
        let parser = ConsoleLogParser::new();

        // Trying to fool the regex by having a name that contain a single " and a [steamid].
        // "aftershave" [U:1:13962573]"
        let line = r#"11/07/2020 - 10:33:44: #     66 "aftershave" [U:1:13962573]" [U:1:13962573] 00:21   60    0 active"#;

        let info = parser.parse_line(line);
        println!("{:#?}", info);
        assert_eq!(
            info,
            LogLineInfo::PlayerInfo {
                id: 66,
                steam_id: "U:1:13962573".to_string(),
                name: r#"aftershave" [U:1:13962573]"#.to_string()
            }
        );
    }

    #[test]
    fn test_parse_console_log() {
        let lines = CONSOLE_OUTPUT_1.lines();
        let parser = ConsoleLogParser::new();

        let mut nothing_rows = 0;
        let mut player_rows = 0;
        for line in lines {
            let info = parser.parse_line(line);
            // println!("console info: {}", line);
            println!("console info: {:?}", info);
            match info {
                LogLineInfo::Nothing => nothing_rows += 1,
                LogLineInfo::PlayerInfo { steam_id, name, id } => player_rows += 1,
            }
        }

        assert_eq!(nothing_rows, 10);
        assert_eq!(player_rows, 23);
    }

    const CONSOLE_OUTPUT_1: &str = r#"
11/07/2020 - 08:41:39: players : 22 humans, 0 bots (32 max)
11/07/2020 - 08:41:39: edicts  : 1280 used of 2048 max
11/07/2020 - 08:41:39: # userid name                uniqueid            connected ping loss state
11/07/2020 - 08:41:39: #     49 "leonid_tea"        [U:1:1023858720]    32:26      121    0 active
11/07/2020 - 08:41:39: #     73 "nYYPPA =D"         [U:1:76603094]      08:28       60    0 active
11/07/2020 - 08:41:39: #     51 "spy"               [U:1:1043180893]    32:21      133    0 active
11/07/2020 - 08:41:39: #      5 "˓H̶ұ̶ャē̶˒" [U:1:863917153]     1:14:05    87    0 active
11/07/2020 - 08:41:39: #     74 "B E Z D A R N O S T" [U:1:92030498]    08:25       54    0 active
11/07/2020 - 08:41:39: #     75 "Captain Condom"    [U:1:137265740]     08:24       70    0 active
11/07/2020 - 08:41:39: #     87 "M1RHO"             [U:1:132949820]     00:44      202   61 spawning
11/07/2020 - 08:41:39: #     76 "joginek"           [U:1:897412264]     08:22      124    0 active
11/07/2020 - 08:41:39: #     72 "ZERO_TWO_"         [U:1:1040486858]    13:18       55    0 active
11/07/2020 - 08:41:39: #     77 "Dümmköpf"        [U:1:257605159]     08:18       96    0 active
11/07/2020 - 08:41:39: #     86 "penÃ"             [U:1:238592473]     00:51       54    0 active
11/07/2020 - 08:41:39: #     58 "S H O R K"         [U:1:343450575]     31:54       66    0 active
11/07/2020 - 08:41:39: #     85 "aftershave"        [U:1:13962573]      01:44       44    0 active
11/07/2020 - 10:33:44: #     66 "aftershave" [U:1:13962573]" [U:1:13962573] 00:21   60    0 active
11/07/2020 - 08:41:39: #     60 "Russoff TRADEIT.GG" [U:1:409399338]    31:43       76    0 active
11/07/2020 - 08:41:39: #     80 "gummiber"          [U:1:994307351]     08:02       88    0 active
11/07/2020 - 08:41:39: #     81 "BORIS the Dwarf"   [U:1:186983432]     07:57       74    0 active
11/07/2020 - 08:41:39: #     82 "Mr Andrew"         [U:1:338468516]     07:55       90    0 active
11/07/2020 - 08:41:39: #     43 "ilia_v_igre"       [U:1:1006898116]    46:44      146    0 active
11/07/2020 - 08:41:39: #     22 "^>the doctor<^"    [U:1:319650194]      1:13:29    78    0 active
11/07/2020 - 08:41:39: #     84 "Lysander"          [U:1:75655188]      04:11       74    0 active
11/07/2020 - 08:41:39: #     41 "Reeves"            [U:1:1007095694]    50:39       71    0 active
11/07/2020 - 08:41:39: #     66 "derfisch06"        [U:1:203358569]     28:26       89    0 active
11/07/2020 - 08:41:40: Msg from 185.25.180.167:27041: svc_UserMessage: type 25, bytes 3
11/07/2020 - 08:41:40: Model '(null)' doesn't have attachment 'backblast' to attach particle system 'rocketbackblast' to.
11/07/2020 - 08:41:40: Msg from 185.25.180.167:27041: svc_UserMessage: type 71, bytes 1
11/07/2020 - 08:41:41: Msg from 185.25.180.167:27041: svc_UserMessage: type 71, bytes 1
11/07/2020 - 08:41:41: Msg from 185.25.180.167:27041: svc_UserMessage: type 71, bytes 1
11/07/2020 - 08:41:42: Model '(null)' doesn't have attachment 'backblast' to attach particle sys
"#;
}
