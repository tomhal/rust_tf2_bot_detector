use serde::{Deserialize, Serialize};

/// The player info we need from Steam Web API.
/// Not all fields are of interest so this struct
/// only contain those we are interested in.
/// See official documentation at:
/// https://wiki.teamfortress.com/wiki/WebAPI/GetPlayerSummaries
#[derive(Serialize, Deserialize, Debug)]
pub struct SteamPlayer {
    #[serde(rename = "steamid")]
    steam_id: String,
    #[serde(rename = "personaname")]
    persona_name: String,
    #[serde(rename = "timecreated")]
    time_created: u64,
    #[serde(rename = "avatarhash")]
    avatar_hash: String,
    avatar: String,
}

/// Helper struct when deserializing the reply from Steam Web API
#[derive(Serialize, Deserialize, Debug)]
struct GetPlayerSummariesBody {
    response: GetPlayerSummariesResponse,
}

/// Helper struct when deserializing the reply from Steam Web API
#[derive(Serialize, Deserialize, Debug)]
struct GetPlayerSummariesResponse {
    players: Vec<SteamPlayer>,
}

impl SteamPlayer {
    /// Deserializes the JSON reply from Steam Web API.
    /// The reply contain several SteamPlayer.
    /// See unit test test_parse_steam_player() for how it looks.
    pub fn from_json_str(json: &str) -> Vec<SteamPlayer> {
        let body: GetPlayerSummariesBody = serde_json::from_str(&json).unwrap();
        body.response.players
    }
}

pub struct SteamWebApiClient {
    pub api_key: String,
}

/// SteamWepApiClient - implements a few of the methods in the Steam Web API.
/// Read more at: https://wiki.teamfortress.com/wiki/WebAPI
impl SteamWebApiClient {
    pub fn new(api_key: String) -> SteamWebApiClient {
        SteamWebApiClient { api_key }
    }

    /// Ask Steam Web API for player info about a list of steam ids.
    /// See official documentation at:
    /// https://wiki.teamfortress.com/wiki/WebAPI/GetPlayerSummaries
    pub fn get_player_summaries(
        &self,
        steam_ids: Vec<String>,
    ) -> Result<Vec<SteamPlayer>, Box<dyn std::error::Error>> {
        let url = format!(
            "http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}",
            self.api_key,
            steam_ids.join(",").as_str()
        );

        let json = reqwest::blocking::get(&url)?.text()?;

        Ok(SteamPlayer::from_json_str(json.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_steam_player() {
        let json = "{\"response\":{\"players\":[{\"steamid\":\"76561197974228301\",\"communityvisibilitystate\":3,\"profilestate\":1,\"personaname\":\"aftershave\",\"commentpermission\":1,\"profileurl\":\"https://steamcommunity.com/profiles/76561197974228301/\",\"avatar\":\"https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/f3/f39ba23bc07d2de9b77abcabae13ee2541f9c938.jpg\",\"avatarmedium\":\"https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/f3/f39ba23bc07d2de9b77abcabae13ee2541f9c938_medium.jpg\",\"avatarfull\":\"https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/f3/f39ba23bc07d2de9b77abcabae13ee2541f9c938_full.jpg\",\"avatarhash\":\"f39ba23bc07d2de9b77abcabae13ee2541f9c938\",\"lastlogoff\":1604400356,\"personastate\":1,\"realname\":\"Ask if you want to know\",\"primaryclanid\":\"103582791432581798\",\"timecreated\":1108579667,\"personastateflags\":0,\"loccountrycode\":\"SE\",\"locstatecode\":\"28\",\"loccityid\":43694}]}}";
        let x = SteamPlayer::from_json_str(json);

        assert!(x.len() == 1);
        assert!(x[0].steam_id == "76561197974228301");
        assert!(x[0].avatar_hash == "f39ba23bc07d2de9b77abcabae13ee2541f9c938");
    }
}
