use chrono::{DateTime, FixedOffset};
use std::convert::From;

use super::{Bookmark, Page, Track};
use crate::graphql::schema::Context;

#[derive(Debug, Clone)]
/// A synchrotron course document
pub struct Document {
    // ID of the document
    pub id: i32,
    // Title of the document
    pub title: String,
    // Timestamp for when the document was created
    pub created_at: String,
    // Timestamp for when the document was last updated
    pub updated_at: String,
}

#[juniper::graphql_object(Context = Context)]
impl Document {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn created_at(&self) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339(self.created_at.as_str()).unwrap()
    }

    pub fn updated_at(&self) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339(self.updated_at.as_str()).unwrap()
    }

    pub async fn bookmarks(&self, limit: i32, offset: i32, context: &Context) -> Vec<Bookmark> {
        context
            .bookmark_data
            .as_ref()
            .unwrap()
            .document_bookmarks(self.id, limit, offset)
            .await
            .unwrap()
    }

    pub async fn pages(&self, limit: i32, offset: i32, context: &Context) -> Vec<Page> {
        context
            .page_data
            .as_ref()
            .unwrap()
            .document_pages(self.id, limit, offset)
            .await
            .unwrap()
    }

    pub async fn tracks(&self, limit: i32, offset: i32, context: &Context) -> Vec<Track> {
        context
            .track_data
            .as_ref()
            .unwrap()
            .document_tracks(self.id, limit, offset)
            .await
            .unwrap()
    }
}

impl From<schema::courses::Document> for Document {
    fn from(x: schema::courses::Document) -> Document {
        Document {
            id: x.id,
            title: x.title,
            created_at: x.created_at,
            updated_at: x.updated_at,
        }
    }
}
