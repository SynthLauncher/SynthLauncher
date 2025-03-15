use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

use super::platform::{Os, OsName};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleActionType {
    Allow,
    Disallow,
}

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Features {
    IsDemoUser,
    HasCustomResolution,
    HasQuickPlaysSupport,
    IsQuickPlaySingleplayer,
    IsQuickPlayMultiplayer,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub action: RuleActionType,
    pub features: Option<HashMap<Features, bool>>,
    pub os: Option<Os>,
}

impl Rule {
    fn matches(&self) -> bool {
        (self.os.is_none() || self.os.as_ref().is_some_and(|os| os.matches()))
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
    // NOTE: These 3 may not be present everywhere
    pub id: Option<String>,
    pub path: Option<PathBuf>,
    pub total_size: Option<i32>,
    
    pub url: String,
    pub sha1: String,
    pub size: i32
}

#[derive(Debug, Deserialize)]
pub struct Downloads {
    pub client: Download,
    pub client_mappings: Download,
    pub server: Download,
    pub server_mappings: Download
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ArgumentValue {
    Value(String),
    Values(Vec<String>)
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Argument {
    Arg(String),
    Rule {
        rules: Vec<Rule>,
        value: ArgumentValue
    }
}

impl Argument {
    fn into_str_args(self) -> Vec<String> {
        match self {
            Argument::Arg(arg) => vec![arg],
            Argument::Rule { rules, value } => {
                if rules.iter().all(Rule::is_allowed) {
                    match value {
                        ArgumentValue::Value(value) => vec![value],
                        ArgumentValue::Values(values) => values
                    }   
                } else {
                    vec![]
                }
            }
        }
    }   
}

#[derive(Debug, Deserialize)]
pub enum Arguments {
    Args {
        game: Vec<Argument>,
        jvm: Vec<Argument>
    },
    MinecraftArgs(String)
}

impl Arguments {
    pub fn into_str_args(self) -> (Vec<String>, Vec<String>) {
        match self {
            Arguments::Args { game, jvm } => {
                let jvm = jvm.into_iter().map(Argument::into_str_args).flatten().collect();
                let game = game.into_iter().map(Argument::into_str_args).flatten().collect();
                (jvm, game) 
            },
            Arguments::MinecraftArgs(args) => {
                let game = args.split(' ').map(|arg| arg.to_string()).collect();

                let jvm = [
                    "-Djava.library.path=${natives_directory}",
                    "-cp",
                    r"${classpath}",
                ];
                let jvm = jvm.into_iter().map(|x| x.to_string()).collect();

                (jvm, game)
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JavaVersion {
    pub component: String,
    pub major_version: u16
}

#[derive(Debug, Deserialize)]
pub struct LibraryDownload {
    pub artifact: Option<Download>,
    pub classifiers: Option<HashMap<String, Download>>
}

#[derive(Debug, Deserialize)]
pub struct Extract {
    pub exlude: Option<Vec<PathBuf>>
}

pub type NativesType = HashMap<OsName, String>;

#[derive(Debug, Deserialize)]
pub struct Library {
    pub downloads: LibraryDownload,
    pub extract: Option<Extract>,
    pub natives: Option<NativesType>,
    pub rules: Option<Vec<Rule>>
}

impl Library {
    pub fn is_allowed(&self) -> bool {
        self.rules.is_none()
            || self
                .rules
                .as_ref()
                .is_some_and(|rules| rules.iter().all(Rule::is_allowed))
    }

    pub fn native_from_platform(&self) -> Option<&Download> {
        let natives = self.natives.as_ref()?;
        let classifiers = self.downloads.classifiers.as_ref()?;

        let mut results = natives
            .iter()
            .filter(|(os, _)| **os == crate::OS)
            .map(|(_, native)| classifiers.get(native).unwrap());

        results.next()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    #[serde(alias = "miencraftArguments")]
    pub arguments: Arguments,
    pub libraries: Vec<Library>,
    pub java_version: Option<JavaVersion>,
    pub main_class: String,
    pub downloads: Downloads,
    pub assets: String,
    pub asset_index: Download
}

impl Client {
    pub fn libraries(&self) -> impl Iterator<Item = &Library> {
        self.libraries.iter().filter(|x| x.is_allowed())
    }
}

