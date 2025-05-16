use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use super::{version_manifest::VersionType, JavaClassName, Os, OsName};

#[derive(Debug, Deserialize)]
pub struct AssetObject {
    pub hash: String,
    pub size: usize,
}

#[derive(Debug, Deserialize)]
pub struct AssetIndex {
    pub objects: HashMap<String, AssetObject>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum RuleActionType {
    Allow,
    Disallow,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Rule {
    pub action: RuleActionType,
    pub features: Option<HashMap<String, bool>>,
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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Download {
    pub path: Option<PathBuf>,
    pub url: String,
    pub sha1: Option<String>,
    pub size: Option<i32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Downloads {
    pub client: Download,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum ArgumentValue {
    Value(String),
    Values(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Argument {
    Arg(String),
    Rule {
        rules: Vec<Rule>,
        value: ArgumentValue,
    },
}

impl Argument {
    fn into_raw(self) -> Vec<String> {
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

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Arguments {
    /// Modern arguments.
    Args {
        game: Vec<Argument>,
        jvm: Vec<Argument>,
    },
    /// Older versions of  Minecraft arguments.
    MinecraftArgs(String),
}

impl Arguments {
    /// Concatenates two arguments.
    /// panics if the arguments are not of the same type.
    pub fn concat(self, other: Self) -> Self {
        match (self, other) {
            (
                Self::Args { game, jvm },
                Self::Args {
                    game: game2,
                    jvm: jvm2,
                },
            ) => Self::Args {
                game: [game, game2].concat(),
                jvm: [jvm, jvm2].concat(),
            },
            (Self::MinecraftArgs(args1), Self::MinecraftArgs(args2)) => {
                Self::MinecraftArgs(format!("{args1} {args2}"))
            }
            _ => unimplemented!("cannot join arguments not of the same type"),
        }
    }

    pub fn into_raw(self) -> (Vec<String>, Vec<String>) {
        match self {
            Arguments::Args { game, jvm } => {
                let jvm: Vec<String> = jvm.into_iter().map(Argument::into_raw).flatten().collect();

                let game = game.into_iter().map(Argument::into_raw).flatten().collect();
                (jvm, game)
            }
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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JavaVersion {
    pub component: String,
    pub major_version: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LibraryDownload {
    pub artifact: Option<Download>,
    pub classifiers: Option<HashMap<String, Download>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Extract {
    pub exclude: Option<Vec<PathBuf>>,
}

pub type Natives = HashMap<OsName, String>;
#[derive(Debug, Deserialize, Clone)]
pub struct Library {
    pub name: JavaClassName,
    pub downloads: LibraryDownload,
    pub extract: Option<Extract>,
    pub natives: Option<Natives>,
    pub rules: Option<Vec<Rule>>,
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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    #[serde(alias = "minecraftArguments")]
    pub arguments: Arguments,
    pub libraries: Vec<Library>,
    pub java_version: Option<JavaVersion>,
    pub main_class: String,
    pub downloads: Downloads,
    pub assets: String,
    pub asset_index: Download,
    pub id: String,
    pub release_time: String,
    pub r#type: VersionType,
}

impl Client {
    pub fn libraries(&self) -> impl Iterator<Item = &Library> {
        self.libraries.iter().filter(|x| x.is_allowed())
    }
}
