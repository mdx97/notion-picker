use anyhow::Result;
use async_trait::async_trait;
use notion::ids::{AsIdentifier, DatabaseId};
use notion::models::search::DatabaseQuery;
use notion::models::{ListResponse, Page};
use notion::NotionApi;

use super::Collector;

/// Gets all database entries from a database with the given id.
pub struct GetAllDatabaseEntriesCollector<'a> {
    /// The Notion API client.
    notion: &'a NotionApi,
    /// The database id to list entries for.
    database: DatabaseId,
    /// The pages that were found in the database.
    pages: Vec<Page>,
}

impl<'a> GetAllDatabaseEntriesCollector<'a> {
    /// Creates a new instance of [`GetAllDatabaseEntriesCollector`].
    pub fn new(notion: &'a NotionApi, database: DatabaseId) -> Self {
        Self {
            notion,
            database,
            pages: Vec::new(),
        }
    }
}

#[async_trait]
impl<'a> Collector for GetAllDatabaseEntriesCollector<'a> {
    type Yield = Vec<Page>;

    type ApiItem = Page;

    async fn fetch(&self) -> Result<ListResponse<Self::ApiItem>> {
        self.notion
            .query_database(self.database.as_id(), DatabaseQuery::default())
            .await
            .map_err(Into::into)
    }

    fn process(&mut self, response: ListResponse<Self::ApiItem>) {
        self.pages.extend(response.results)
    }

    fn finish(self) -> Self::Yield {
        self.pages
    }
}
