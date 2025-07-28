use std::fmt;

use serde::{Deserialize, Serialize};
use sl_utils::{dlog, errors::BackendError, requester::Requester};

#[derive(Debug, Deserialize, Serialize)]
pub struct ModrinthSearchHit {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub project_type: String,
    pub downloads: u32,
    pub icon_url: Option<String>,
    pub project_id: String,
    pub author: String,
    pub versions: Vec<String>,
    pub follows: u32,
    pub latest_version: String,
    pub gallery: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResult {
    pub hits: Vec<ModrinthSearchHit>,
    pub total_hits: u32
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FacetType {
    ProjectType,
    Versions,
    Title,
    Author,
    Follows,
    ProjectId,
    Downloads,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FacetOp {
    Eq,  // :
    Neq, // !=
    Gte, // >=
    Gt,  // >
    Lte, // <=
    Lt,  // <
}

impl From<&str> for FacetOp {
    fn from(op: &str) -> Self {
        match op {
            "==" => FacetOp::Eq,
            "!=" => FacetOp::Neq,
            ">" => FacetOp::Gt,
            "<" => FacetOp::Lt,
            ">=" => FacetOp::Gte,
            "<=" => FacetOp::Lte,
            _ => panic!("Unsupported FacetOp string: {}", op),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FacetFilter {
    pub facet: FacetType,
    pub op: FacetOp,
    pub value: String,
}

impl fmt::Display for FacetOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            FacetOp::Eq => ":",
            FacetOp::Neq => "!=",
            FacetOp::Gte => ">=",
            FacetOp::Gt => ">",
            FacetOp::Lte => "<=",
            FacetOp::Lt => "<",
        };
        write!(f, "{}", symbol)
    }
}

impl fmt::Display for FacetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            FacetType::ProjectType => "project_type",
            FacetType::Versions => "versions",
            FacetType::Title => "title",
            FacetType::Author => "author",
            FacetType::Follows => "follows",
            FacetType::ProjectId => "project_id",
            FacetType::Downloads => "downloads",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for FacetFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.facet, self.op, self.value)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    pub query: Option<String>,
    pub facets: Option<Vec<Vec<FacetFilter>>>,
    pub index: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl Params {
    pub fn new(
        query: Option<String>,
        facets: Option<Vec<Vec<FacetFilter>>>,
        index: Option<String>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Self {
        Params {
            query,
            facets,
            index,
            limit,
            offset,
        }
    }
}

#[macro_export]
macro_rules! facet_filters {
    (
        $( [ $($facet:ident $op:tt $val:expr),+ ] ),+
    ) => {{
        use $crate::modrinth::api::search::{FacetFilter, FacetType, FacetOp};
        let mut filters = Vec::new();

        $(
            let mut group = Vec::new();
            $(
                let op = stringify!($op).into();

                group.push(FacetFilter {
                    facet: FacetType::$facet,
                    op,
                    value: $val.to_string(),
                });
            )+

            filters.push(group);
        )+

        filters
    }};
}

pub fn build_facets(facets: &Vec<Vec<FacetFilter>>) -> Result<Option<String>, BackendError> {
    if facets.is_empty() {
        return Ok(None);
    }

    let mut json_facets: Vec<Vec<String>> = Vec::new();

    for group in facets {
        if !group.is_empty() {
            json_facets.push(group.iter().map(|f| f.to_string()).collect());
        }
    }

    if json_facets.is_empty() {
        return Ok(None);
    }

    Ok(Some(serde_json::to_string(&json_facets)?))
}

pub async fn query_search(params: Params) -> Result<SearchResult, BackendError> {
    let mut query_parts = Vec::new();

    if let Some(query) = params.query {
        if !query.trim().is_empty() {
            query_parts.push(format!("query={}", &query));
        }
    }

    if let Some(index) = params.index {
        query_parts.push(format!("index={}", &index));
    }

    if let Some(limit) = params.limit {
        query_parts.push(format!("limit={}", &limit));
    }

    if let Some(offset) = params.offset {
        query_parts.push(format!("offset={}", offset));
    }

    if let Some(facets) = params.facets {
        if let Some(facets_str) = build_facets(&facets)? {
            query_parts.push(format!("facets={}", &facets_str));
        }
    }

    let url = if query_parts.is_empty() {
        "https://api.modrinth.com/v2/search".to_string()
    } else {
        format!(
            "https://api.modrinth.com/v2/search?{}",
            query_parts.join("&")
        )
    };

    dlog!("{}", url);

    let json = Requester::new().get_json(&url).await?;
    Ok(json)
}
