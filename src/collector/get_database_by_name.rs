use anyhow::Result;
use async_trait::async_trait;
use notion::models::search::NotionSearch;
use notion::models::{Database, ListResponse, Object};
use notion::NotionApi;

use super::Collector;

/// Gets a single database from Notion with the given `name`.
pub struct GetDatabaseByNameCollector<'a> {
    /// The Notion API client.
    notion: &'a NotionApi,
    /// The database name to filter by.
    name: &'a str,
    /// The found database, if any.
    database: Option<Database>,
}

impl<'a> GetDatabaseByNameCollector<'a> {
    /// Creates a new instance of [`GetDatabaseByNameCollector`].
    pub fn new(notion: &'a NotionApi, name: &'a str) -> Self {
        Self {
            notion,
            name,
            database: None,
        }
    }
}

#[async_trait]
impl<'a> Collector for GetDatabaseByNameCollector<'a> {
    type Yield = Option<Database>;

    type ApiItem = Object;

    async fn fetch(&self) -> Result<ListResponse<Self::ApiItem>> {
        self.notion
            .search(NotionSearch::Query(self.name.into()))
            .await
            .map_err(Into::into)
    }

    fn process(&mut self, response: ListResponse<Self::ApiItem>) {
        self.database = response
            .results
            .into_iter()
            .filter_map(|object| {
                if let Object::Database { database } = object {
                    Some(database)
                } else {
                    None
                }
            })
            .next();
    }

    fn done(&self) -> bool {
        self.database.is_some()
    }

    fn finish(self) -> Self::Yield {
        self.database
    }
}
