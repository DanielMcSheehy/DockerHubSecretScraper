use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "page_size")]
    pub page_size: i64,
    pub next: String,
    pub previous: String,
    pub page: i64,
    pub count: i64,
    pub summaries: Vec<Summary>,
}

impl Root {
    pub fn get_summary_names(&self) -> Vec<String> {
        self.summaries.par_iter()
             .map(|s| s.name.clone()).collect()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub id: String,
    pub name: String,
    pub slug: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub publisher: Publisher,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "short_description")]
    pub short_description: String,
    pub source: String,
    pub popularity: i64,
    pub categories: Value,
    #[serde(rename = "operating_systems")]
    pub operating_systems: Value,
    pub architectures: Value,
    #[serde(rename = "logo_url")]
    pub logo_url: Value,
    #[serde(rename = "certification_status")]
    pub certification_status: String,
    #[serde(rename = "star_count")]
    pub star_count: i64,
    #[serde(rename = "filter_type")]
    pub filter_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Publisher {
}
