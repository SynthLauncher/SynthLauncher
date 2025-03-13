use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

use super::platform::Os;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleActionType {
    Allow,
    Disallow,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub action: RuleActionType,
    pub features: Option<HashMap<String, bool>>,
    pub os: Option<Os>,
}

impl Rule {
    fn matches(&self) -> bool {
        (self.os.is_none() || self.os.as_ref().is_some_and(|os| os.if_matches()))
            && self.features.is_none()
    }

    pub fn is_allowed(&self) -> bool {
        let matched = self.matches();
        match self.action {
            RuleActionType::Allow => matched,
            RuleActionType::Disallow => !matched,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Download {
    pub path: Option<PathBuf>,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Downloads {
    pub client: Download,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ArgumentValue {
    Value(String),
    Values(Vec<String>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Argument {
    Arg(String),
    Rule {
        rules: Vec<Rule>,
        value: ArgumentValue,
    },
}

impl Argument {
    fn to_json(self) -> Vec<String> {
        match self {
            Argument::Arg(arg) => vec![arg],
            Argument::Rule { rules, value } => {
                if rules.iter().all(Rule::is_allowed) {
                    match value {
                        ArgumentValue::Value(value) => vec![value],
                        ArgumentValue::Values(values) => values,
                    }
                } else {
                    vec![]
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Arguments {
    Args {
        game: Vec<Argument>,
        jvm: Vec<Argument>
    },
    MinecraftArgs(String)
}

impl Arguments {
    
}