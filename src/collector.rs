use anyhow::Result;
use async_trait::async_trait;
use notion::models::ListResponse;

mod get_all_database_entries;
mod get_database_by_name;

pub use get_all_database_entries::*;
pub use get_database_by_name::*;

/// A trait that defines the different lifecycle methods of "collecting" a multi-page data source
/// from the Notion API.
#[async_trait]
pub trait Collector: Sized + Send + Sync {
    /// The final return type of this trait.
    type Yield;

    /// The type that [`Collector::fetch`] gets from the Notion API.
    type ApiItem;

    /// This method is the primary driver of this trait and actually performs the process of
    /// collecting all pages of data and returning it.
    async fn collect(mut self) -> Result<Self::Yield> {
        loop {
            let response = self.fetch().await?;
            let has_more = response.has_more;
            self.process(response);
            if self.done() || !has_more {
                break;
            }
        }
        Ok(self.finish())
    }

    /// A wrapper method for the API call made to Notion to gather each page.
    async fn fetch(&self) -> Result<ListResponse<Self::ApiItem>>;

    /// A function that processes each page of data from the Notion API.
    fn process(&mut self, response: ListResponse<Self::ApiItem>);

    /// Optionally allow the programmer to abort early in some cases.
    fn done(&self) -> bool {
        false
    }

    /// The final return value of this trait.
    fn finish(self) -> Self::Yield;
}
