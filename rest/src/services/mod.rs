pub mod movie;
pub mod theatre;
pub mod user;

use serde::Deserialize;


#[derive(Deserialize, Copy, Clone)]
pub enum SortBy {
    #[serde(alias = "newest")]
    Newest,
    #[serde(alias = "oldest")]
    Oldest,
}