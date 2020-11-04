// https://github.com/PazerOP/SourceRCON
// https://github.com/Subtixx/source-rcon-library

#[derive(Debug)]
pub struct RConArgs {
    pub password: String,
    pub port: u16,
}

impl RConArgs {
    pub fn new() -> Self {
        RConArgs {
            password: "rconpwd".to_string(),
            port: 41234,
        }
    }
}

#[derive(Debug)]
pub struct RCon {}

impl RCon {}
