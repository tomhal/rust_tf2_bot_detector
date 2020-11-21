use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

use crate::player::PlayerInfo;

#[derive(Serialize, Deserialize, Debug)]
pub struct RulesFile {
    #[serde(rename = "$schema")]
    schema: String,
    file_info: FileInfo,
    rules: Vec<Rule>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    authors: Vec<String>,
    description: String,
    title: String,
    update_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {
    actions: RuleAction,
    description: String,
    triggers: Trigger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trigger {
    #[serde(default)]
    mode: TriggerMode,
    username_text_match: Option<TextMatch>,
    chatmsg_text_match: Option<TextMatch>,
    avatar_match: Option<Vec<AvatarMatch>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TriggerMode {
    MatchAll,
    MatchAny,
}

impl Default for TriggerMode {
    fn default() -> Self {
        TriggerMode::MatchAll
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TextMatch {
    case_sensitive: bool,
    mode: TextMatchMode,
    patterns: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AvatarMatch {
    avatar_hash: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TextMatchMode {
    Equal,
    Contains,
    StartsWith,
    EndsWith,
    Regex,
    Word,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RuleAction {
    #[serde(default)]
    mark: Vec<PlayerAttribute>,
    #[serde(default)]
    unmark: Vec<PlayerAttribute>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum PlayerAttribute {
    Cheater,
    Suspicious,
    Exploiter,
    Racist,
}

#[derive(Debug)]
pub struct RuleFileMatchResult {
    mark_actions: HashSet<PlayerAttribute>,
    unmark_actions: HashSet<PlayerAttribute>,
}

impl RulesFile {
    pub fn from_file(filename: &str) -> RulesFile {
        let mut f = File::open(filename).unwrap();
        let mut json = String::new();
        f.read_to_string(&mut json).unwrap();
        RulesFile::from_json_str(&json)
    }

    pub fn from_json_str(json: &str) -> RulesFile {
        serde_json::from_str(&json).unwrap()
    }

    pub fn get_actions(&self, player: &PlayerInfo, chat_text: &str) -> RuleFileMatchResult {
        let mut mark_actions: HashSet<PlayerAttribute> = HashSet::new();
        let mut unmark_actions: HashSet<PlayerAttribute> = HashSet::new();

        for rule in self.rules.iter() {
            if rule.triggers.is_match(&player, chat_text) {
                for &action in rule.actions.mark.as_slice() {
                    mark_actions.insert(action);
                }
                for &action in rule.actions.unmark.iter() {
                    unmark_actions.insert(action);
                }
            }
        }

        RuleFileMatchResult {
            mark_actions,
            unmark_actions,
        }
    }
}

impl TextMatch {
    fn is_match(&self, text: &str) -> bool {
        let mut patterns: Vec<String> = Vec::with_capacity(self.patterns.len());

        match self.mode {
            TextMatchMode::Equal => {
                for p in self.patterns.iter() {
                    let pattern = format!(r"^{}$", regex::escape(p.as_str()));
                    patterns.push(pattern);
                }
            }
            TextMatchMode::Contains => {
                for p in self.patterns.iter() {
                    let pattern = format!(r"{}", regex::escape(p.as_str()));
                    patterns.push(pattern);
                }
            }
            TextMatchMode::StartsWith => {
                for p in self.patterns.iter() {
                    let pattern = format!(r"^{}", regex::escape(p.as_str()));
                    patterns.push(pattern);
                }
            }
            TextMatchMode::EndsWith => {
                for p in self.patterns.iter() {
                    let pattern = format!(r"{}$", regex::escape(p.as_str()));
                    patterns.push(pattern);
                }
            }
            TextMatchMode::Regex => {
                for p in self.patterns.iter() {
                    patterns.push(p.to_string());
                }
            }
            TextMatchMode::Word => {
                for p in self.patterns.iter() {
                    let pattern = format!(r"(^|\W){}($|\W)", regex::escape(p.as_str()));
                    patterns.push(pattern);
                }
            }
        }

        let mut builder = regex::RegexSetBuilder::new(patterns);
        builder.case_insensitive(!self.case_sensitive);
        let x = builder.build().unwrap();

        x.is_match(text)
    }
}

impl AvatarMatch {
    fn is_match(&self, player: &PlayerInfo) -> bool {
        self.avatar_hash == player.avatar_hash
    }
}

impl Trigger {
    fn is_match(&self, player: &PlayerInfo, chat_text: &str) -> bool {
        match self.mode {
            TriggerMode::MatchAll => self.match_all(player, chat_text),
            TriggerMode::MatchAny => self.match_any(player, chat_text),
        }
    }

    fn match_all(&self, player: &PlayerInfo, chat_text: &str) -> bool {
        match &self.username_text_match {
            Some(textmatch) => {
                if !textmatch.is_match(player.nickname.as_str()) {
                    return false;
                }
            }
            None => {}
        }

        match &self.chatmsg_text_match {
            Some(textmatch) => {
                if !textmatch.is_match(chat_text) {
                    return false;
                }
            }
            None => {}
        }

        match &self.avatar_match {
            Some(avatar_hashes) => {
                for avatar_hash in avatar_hashes.iter() {
                    if !avatar_hash.is_match(player) {
                        return false;
                    }
                }
            }
            None => {}
        }

        true
    }

    fn match_any(&self, player: &PlayerInfo, chat_text: &str) -> bool {
        match &self.username_text_match {
            Some(textmatch) => {
                if textmatch.is_match(player.nickname.as_str()) {
                    return true;
                }
            }
            None => {}
        }

        match &self.chatmsg_text_match {
            Some(textmatch) => {
                if textmatch.is_match(chat_text) {
                    return true;
                }
            }
            None => {}
        }

        match &self.avatar_match {
            Some(avatar_hashes) => {
                for avatar_hash in avatar_hashes.iter() {
                    if avatar_hash.is_match(player) {
                        return true;
                    }
                }
            }
            None => {}
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that all conditions in a Trigger, if set, must be true for the Trigger
    // to be considered matching.
    #[test]
    fn test_trigger_match_all() {
        let player = PlayerInfo {
            avatar_hash: "avatarhash".to_string(),
            nickname: "username".to_string(),
            steamd_id: "steamid".to_string(),
        };

        let matches_nothing = Some(TextMatch {
            mode: TextMatchMode::Equal,
            case_sensitive: false,
            patterns: vec!["nothing".to_string()],
        });

        let chattext_textmatch = Some(TextMatch {
            mode: TextMatchMode::Equal,
            case_sensitive: false,
            patterns: vec!["chat".to_string()],
        });

        let username_textmatch = Some(TextMatch {
            mode: TextMatchMode::Equal,
            case_sensitive: false,
            patterns: vec!["username".to_string()],
        });

        let avatarmatch = Some(vec![AvatarMatch {
            avatar_hash: "avatarhash".to_string(),
        }]);

        let chat_text = "chat";

        let mut trigger = Trigger {
            mode: TriggerMode::MatchAll,
            avatar_match: None,
            chatmsg_text_match: None,
            username_text_match: None,
        };

        assert!(trigger.is_match(&player, chat_text));

        trigger.avatar_match = avatarmatch.clone();
        assert!(trigger.is_match(&player, chat_text));

        trigger.avatar_match = None;
        trigger.chatmsg_text_match = chattext_textmatch.clone();
        assert!(trigger.is_match(&player, chat_text));

        trigger.avatar_match = None;
        trigger.chatmsg_text_match = None;
        trigger.username_text_match = username_textmatch.clone();
        assert!(trigger.is_match(&player, chat_text));

        trigger.avatar_match = avatarmatch.clone();
        trigger.chatmsg_text_match = chattext_textmatch.clone();
        trigger.username_text_match = username_textmatch.clone();
        assert!(trigger.is_match(&player, chat_text));

        trigger.avatar_match = avatarmatch.clone();
        trigger.chatmsg_text_match = chattext_textmatch.clone();
        trigger.username_text_match = matches_nothing.clone();
        assert!(!trigger.is_match(&player, chat_text));
    }

    // Test that at least one matcher in the Trigger needs to be true
    // for the Trigger to be considered matching.
    // TODO: This is more or less just a copy of test_trigger_match_all, make a better version.
    #[test]
    fn test_trigger_match_any() {
        let player = PlayerInfo {
            avatar_hash: "avatarhash".to_string(),
            nickname: "username".to_string(),
            steamd_id: "steamid".to_string(),
        };

        let matches_nothing = Some(TextMatch {
            mode: TextMatchMode::Equal,
            case_sensitive: false,
            patterns: vec!["nothing".to_string()],
        });

        let chattext_textmatch = Some(TextMatch {
            mode: TextMatchMode::Equal,
            case_sensitive: false,
            patterns: vec!["chat".to_string()],
        });

        let username_textmatch = Some(TextMatch {
            mode: TextMatchMode::Equal,
            case_sensitive: false,
            patterns: vec!["username".to_string()],
        });

        let avatarmatch = Some(vec![AvatarMatch {
            avatar_hash: "avatarhash".to_string(),
        }]);

        let chat_text = "chat";

        let mut trigger = Trigger {
            mode: TriggerMode::MatchAny,
            avatar_match: None,
            chatmsg_text_match: matches_nothing.clone(),
            username_text_match: matches_nothing.clone(),
        };

        // Nothing matches
        assert!(!trigger.is_match(&player, chat_text));

        trigger.avatar_match = avatarmatch.clone();
        assert!(trigger.is_match(&player, chat_text));

        trigger.chatmsg_text_match = chattext_textmatch.clone();
        assert!(trigger.is_match(&player, chat_text));

        trigger.avatar_match = None;
        trigger.chatmsg_text_match = None;
        trigger.username_text_match = username_textmatch.clone();
        assert!(trigger.is_match(&player, chat_text));

        trigger.avatar_match = avatarmatch.clone();
        trigger.chatmsg_text_match = chattext_textmatch.clone();
        trigger.username_text_match = username_textmatch.clone();
        assert!(trigger.is_match(&player, chat_text));

        trigger.avatar_match = None;
        trigger.chatmsg_text_match = chattext_textmatch.clone();
        trigger.username_text_match = matches_nothing.clone();
        assert!(trigger.is_match(&player, chat_text));
    }

    #[test]
    fn test_textmatch_equal() {
        let mut textmatch = TextMatch {
            case_sensitive: false,
            mode: TextMatchMode::Equal,
            patterns: vec!["a".to_string()],
        };
        assert_eq!(textmatch.is_match("a"), true);
        assert_eq!(textmatch.is_match("A"), true);
        assert_eq!(textmatch.is_match("B"), false);
        textmatch.case_sensitive = true;
        assert_eq!(textmatch.is_match("a"), true);
        assert_eq!(textmatch.is_match("A"), false);
        assert_eq!(textmatch.is_match("B"), false);
    }

    #[test]
    fn test_textmatch_contains() {
        let mut textmatch = TextMatch {
            case_sensitive: false,
            mode: TextMatchMode::Contains,
            patterns: vec!["a".to_string()],
        };
        assert_eq!(textmatch.is_match("mamma"), true);
        assert_eq!(textmatch.is_match("mAmmA"), true);
        assert_eq!(textmatch.is_match("B"), false);
        textmatch.case_sensitive = true;
        assert_eq!(textmatch.is_match("mamma"), true);
        assert_eq!(textmatch.is_match("mAmmA"), false);
        assert_eq!(textmatch.is_match("B"), false);
    }

    #[test]
    fn test_textmatch_starts_with() {
        let mut textmatch = TextMatch {
            case_sensitive: false,
            mode: TextMatchMode::StartsWith,
            patterns: vec!["a".to_string()],
        };
        assert_eq!(textmatch.is_match("amma"), true);
        assert_eq!(textmatch.is_match("AmmA"), true);
        assert_eq!(textmatch.is_match("B"), false);
        textmatch.case_sensitive = true;
        assert_eq!(textmatch.is_match("amma"), true);
        assert_eq!(textmatch.is_match("AmmA"), false);
        assert_eq!(textmatch.is_match("B"), false);
    }

    #[test]
    fn test_textmatch_ends_with() {
        let mut textmatch = TextMatch {
            case_sensitive: false,
            mode: TextMatchMode::EndsWith,
            patterns: vec!["a".to_string()],
        };
        assert_eq!(textmatch.is_match("mma"), true);
        assert_eq!(textmatch.is_match("mmA"), true);
        assert_eq!(textmatch.is_match("B"), false);
        textmatch.case_sensitive = true;
        assert_eq!(textmatch.is_match("mma"), true);
        assert_eq!(textmatch.is_match("mmA"), false);
        assert_eq!(textmatch.is_match("B"), false);
    }

    #[test]
    fn test_textmatch_regex() {
        let mut textmatch = TextMatch {
            case_sensitive: false,
            mode: TextMatchMode::Regex,
            patterns: vec!["furry-bot \\d+".to_string()],
        };
        assert_eq!(textmatch.is_match("furry-bot 123"), true);
        assert_eq!(textmatch.is_match("Furry-Bot 123"), true);
        assert_eq!(textmatch.is_match("B"), false);
        textmatch.case_sensitive = true;
        assert_eq!(textmatch.is_match("furry-bot 123"), true);
        assert_eq!(textmatch.is_match("Furry-Bot 123"), false);
        assert_eq!(textmatch.is_match("B"), false);
    }

    #[test]
    fn test_textmatch_word() {
        let textmatch = TextMatch {
            case_sensitive: false,
            mode: TextMatchMode::Word,
            patterns: vec!["nigger".to_string(), "niggers".to_string()],
        };
        assert_eq!(textmatch.is_match("En nigger nogger glass"), true);
        assert_eq!(textmatch.is_match("En niggernoggerglass"), false);
        assert_eq!(textmatch.is_match("nigger"), true);
        assert_eq!(textmatch.is_match("niggernogger"), false);
    }

    #[test]
    fn test_from_json_regex() {
        let json = r#"
        {
            "$schema": "", "file_info": { "authors": [ "" ], "description": "", "title": "", "update_url": "" },
            "rules": [
                {
                    "actions": {
                        "mark": [
                            "cheater"
                        ]
                    },
                    "description": "(catbot) furry-bot",
                    "triggers": {
                        "username_text_match": {
                            "case_sensitive": false,
                            "mode": "regex",
                            "patterns": [
                                "furry-bot \\d+"
                            ]
                        }
                    }
                }
            ]
        }"#;
        let rules_file = RulesFile::from_json_str(json);
        let rules = rules_file.rules;

        assert_eq!(rules.len(), 1);
        let rule = &rules[0];
        assert_eq!(rule.description, "(catbot) furry-bot");
        assert_eq!(rule.actions.mark.len(), 1);
        assert_eq!(rule.actions.mark[0], PlayerAttribute::Cheater);
        assert_eq!(rule.triggers.mode, TriggerMode::MatchAll);
        assert_eq!(rule.triggers.avatar_match, None);
        assert!(rule.triggers.chatmsg_text_match.is_none());
        assert_eq!(
            rule.triggers.username_text_match,
            Some(TextMatch {
                mode: TextMatchMode::Regex,
                case_sensitive: false,
                patterns: vec!["furry-bot \\d+".to_string()]
            })
        );

        let textmatch = rule.triggers.username_text_match.as_ref().unwrap();
        assert_eq!(textmatch.is_match("furry-bot 123"), true);
        assert_eq!(textmatch.is_match("Furry-Bot 123"), true);
        assert_eq!(textmatch.is_match("B"), false);
    }

    #[test]
    fn test_from_json() {
        let json = r#"
        {
            "$schema": "", "file_info": { "authors": [ "" ], "description": "", "title": "", "update_url": "" },
            "rules": [
                {
                    "actions": {
                        "mark": [
                            "cheater"
                        ]
                    },
                    "description": "description",
                    "triggers": {
                        "username_text_match": {
                            "case_sensitive": true,
                            "mode": "contains",
                            "patterns": [
                                "pattern 1",
                                "pattern 2"
                            ]
                        },
                        "avatar_match": [
                            {
                                "avatar_hash": "76c03c7865876dd13dbe4b60aad86150b8fc6233"
                            }
                        ]
                    }
                }
            ]
        }"#;
        let rules_file = RulesFile::from_json_str(json);
        let rules = rules_file.rules;

        assert_eq!(rules.len(), 1);
        let rule = &rules[0];
        assert_eq!(rule.description, "description");
        assert_eq!(rule.actions.mark.len(), 1);
        assert_eq!(rule.actions.mark[0], PlayerAttribute::Cheater);
        assert_eq!(rule.triggers.mode, TriggerMode::MatchAll);
        assert_eq!(
            rule.triggers.avatar_match,
            Some(vec![AvatarMatch {
                avatar_hash: "76c03c7865876dd13dbe4b60aad86150b8fc6233".to_string()
            }])
        );
        assert!(rule.triggers.chatmsg_text_match.is_none());
        assert_eq!(
            rule.triggers.username_text_match,
            Some(TextMatch {
                mode: TextMatchMode::Contains,
                case_sensitive: true,
                patterns: vec!["pattern 1".to_string(), "pattern 2".to_string()]
            })
        );
    }

    #[test]
    fn test_rulefile_get_actions() {
        let json = r#"
        {
            "$schema": "", "file_info": { "authors": [ "" ], "description": "", "title": "", "update_url": "" },
            "rules": [
                {
                    "actions": {
                        "mark": [
                            "cheater"
                        ]
                    },
                    "description": "description",
                    "triggers": {
                        "mode": "match_any",
                        "username_text_match": {
                            "case_sensitive": true,
                            "mode": "contains",
                            "patterns": [
                                "pattern 1",
                                "pattern 2"
                            ]
                        },
                        "avatar_match": [
                            {
                                "avatar_hash": "avatarhash"
                            }
                        ]
                    }
                }
            ]
        }"#;
        let rules_file = RulesFile::from_json_str(json);

        let player = PlayerInfo {
            steamd_id: "steamid".to_string(),
            nickname: "cheaternick".to_string(),
            avatar_hash: "avatarhash".to_string(),
        };
        let chat_text = "git gud";

        let actual = rules_file.get_actions(&player, chat_text);
        assert_eq!(1, actual.mark_actions.len());
        assert_eq!(0, actual.unmark_actions.len());
    }
}
