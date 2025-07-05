use sl_core::HTTP_CLIENT;
use sl_utils::utils::errors::BackendError;
use synrinth::{
    api::search::query_search,
    models::search::{QueryParams, Search},
};

pub async fn search_query_default<'a>(query: QueryParams<'a>) -> Result<Search, BackendError> {
    Ok(query_search(&HTTP_CLIENT, query).await?)
}

