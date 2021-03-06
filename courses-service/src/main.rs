use std::collections::HashMap;
use std::env;

use sqlx::postgres::{PgPoolOptions, Postgres};
use tonic::{transport::Server, Request, Response, Status};

use schema::{
    courses::{
        courses_server::{Courses, CoursesServer},
        Anchor, Bookmark, CreateAnchorRequest, CreateAnchorResponse, CreateBookmarkRequest,
        CreateBookmarkResponse, CreateUserAnchorRequest, CreateUserAnchorResponse,
        DeleteAnchorRequest, DeleteAnchorResponse, DeleteBookmarkRequest, DeleteBookmarkResponse,
        DeleteUserAnchorRequest, DeleteUserAnchorResponse, Document, GetAnchorsByIDsRequest,
        GetAnchorsByIDsResponse, GetAnchorsByPageIDsRequest, GetAnchorsByPageIDsResponse,
        GetBookmarksByIDsRequest, GetBookmarksByIDsResponse, GetDocumentBookmarksRequest,
        GetDocumentBookmarksResponse, GetDocumentPagesRequest, GetDocumentPagesResponse,
        GetDocumentTracksRequest, GetDocumentTracksResponse, GetDocumentsByIDsRequest,
        GetDocumentsByIDsResponse, GetDocumentsRequest, GetDocumentsResponse, GetPagesByIDsRequest,
        GetPagesByIDsResponse, GetTracksByIDsRequest, GetTracksByIDsResponse,
        GetUserAnchorsByIDsRequest, GetUserAnchorsByIDsResponse, GetUserAnchorsByPageIDsRequest,
        GetUserAnchorsByPageIDsResponse, Page, PageAnchors, PageUserAnchors, Track,
        UpdateTrackTitleRequest, UpdateTrackTitleResponse, UserAnchor,
    },
    shared::UserRole,
};

mod errors;

use errors::CoursesServiceError;

#[derive(Debug)]
pub struct CoursesService<T>
where
    for<'a> &'a T: sqlx::Executor<'a, Database = Postgres>,
{
    executor: T,
}

impl<T> CoursesService<T>
where
    for<'a> &'a T: sqlx::Executor<'a, Database = Postgres>,
{
    pub fn new(executor: T) -> Self {
        Self { executor }
    }
}

#[tonic::async_trait]
impl<T: Send + Sync + 'static> Courses for CoursesService<T>
where
    for<'a> &'a T: sqlx::Executor<'a, Database = Postgres>,
{
    async fn get_documents(
        &self,
        request: Request<GetDocumentsRequest>,
    ) -> Result<Response<GetDocumentsResponse>, Status> {
        let req = request.into_inner();
        let documents = sqlx::query!(
            "SELECT * FROM documents LIMIT $1 OFFSET $2;",
            req.limit as i64,
            req.offset as i64
        )
        .fetch_all(&self.executor)
        .await
        .map_err(CoursesServiceError::from)?;

        Ok(Response::new(GetDocumentsResponse {
            total: documents.len() as i32,
            documents: documents
                .into_iter()
                .map(|d| Document {
                    id: d.id,
                    title: d.title,
                    created_at: d.created_at.to_rfc3339(),
                    updated_at: d.updated_at.to_rfc3339(),
                })
                .collect(),
        }))
    }

    async fn get_documents_by_ids(
        &self,
        request: Request<GetDocumentsByIDsRequest>,
    ) -> Result<Response<GetDocumentsByIDsResponse>, Status> {
        let req = request.into_inner();
        let documents = sqlx::query!(
            "SELECT * FROM documents WHERE id IN (SELECT * FROM UNNEST($1::int[]));",
            &req.ids
        )
        .fetch_all(&self.executor)
        .await
        .map_err(CoursesServiceError::from)?;

        Ok(Response::new(GetDocumentsByIDsResponse {
            documents: documents
                .into_iter()
                .map(|d| Document {
                    id: d.id,
                    title: d.title,
                    created_at: d.created_at.to_rfc3339(),
                    updated_at: d.updated_at.to_rfc3339(),
                })
                .collect(),
        }))
    }

    async fn get_document_pages(
        &self,
        request: tonic::Request<GetDocumentPagesRequest>,
    ) -> Result<tonic::Response<GetDocumentPagesResponse>, tonic::Status> {
        let req = request.into_inner();
        let pages = sqlx::query!(
            "SELECT * FROM pages WHERE document=$1 LIMIT $2 OFFSET $3;",
            req.document_id,
            req.limit as i64,
            req.offset as i64
        )
        .fetch_all(&self.executor)
        .await
        .map_err(CoursesServiceError::from)?;

        Ok(Response::new(GetDocumentPagesResponse {
            total: pages.len() as i32,
            pages: pages
                .into_iter()
                .map(|p| Page {
                    id: p.id,
                    page_number: p.page_number,
                    image_path: p.image_path,
                    aspect_ratio: p.aspect_ratio,
                    height: p.height,
                    document_id: p.document,
                })
                .collect(),
        }))
    }

    async fn get_pages_by_ids(
        &self,
        request: tonic::Request<GetPagesByIDsRequest>,
    ) -> Result<tonic::Response<GetPagesByIDsResponse>, tonic::Status> {
        let req = request.into_inner();
        let pages = sqlx::query!(
            "SELECT * FROM pages WHERE id IN (SELECT * FROM UNNEST($1::int[]));",
            &req.ids
        )
        .fetch_all(&self.executor)
        .await
        .map_err(CoursesServiceError::from)?;

        Ok(Response::new(GetPagesByIDsResponse {
            pages: pages
                .into_iter()
                .map(|p| Page {
                    id: p.id,
                    page_number: p.page_number,
                    image_path: p.image_path,
                    aspect_ratio: p.aspect_ratio,
                    height: p.height,
                    document_id: p.document,
                })
                .collect(),
        }))
    }

    async fn get_document_tracks(
        &self,
        request: tonic::Request<GetDocumentTracksRequest>,
    ) -> Result<tonic::Response<GetDocumentTracksResponse>, tonic::Status> {
        let req = request.into_inner();
        let tracks = sqlx::query!(
            "SELECT * FROM tracks WHERE document=$1 LIMIT $2 OFFSET $3;",
            req.document_id,
            req.limit as i64,
            req.offset as i64
        )
        .fetch_all(&self.executor)
        .await
        .map_err(CoursesServiceError::from)?;

        Ok(Response::new(GetDocumentTracksResponse {
            total: tracks.len() as i32,
            tracks: tracks
                .into_iter()
                .map(|t| Track {
                    id: t.id,
                    track_number: t.track_number,
                    title: t.title,
                    audio_path: t.audio_path,
                    document_id: t.document,
                })
                .collect(),
        }))
    }

    async fn get_tracks_by_ids(
        &self,
        request: tonic::Request<GetTracksByIDsRequest>,
    ) -> Result<tonic::Response<GetTracksByIDsResponse>, tonic::Status> {
        let req = request.into_inner();
        let tracks = sqlx::query!(
            "SELECT * FROM tracks WHERE id IN (SELECT * FROM UNNEST($1::int[]));",
            &req.ids
        )
        .fetch_all(&self.executor)
        .await
        .map_err(CoursesServiceError::from)?;

        Ok(Response::new(GetTracksByIDsResponse {
            tracks: tracks
                .into_iter()
                .map(|t| Track {
                    id: t.id,
                    track_number: t.track_number,
                    title: t.title,
                    audio_path: t.audio_path,
                    document_id: t.document,
                })
                .collect(),
        }))
    }

    async fn update_track_title(
        &self,
        request: tonic::Request<UpdateTrackTitleRequest>,
    ) -> Result<tonic::Response<UpdateTrackTitleResponse>, tonic::Status> {
        let req = request.into_inner();

        if let Some(user) = req.active_user {
            if user.role == (UserRole::Moderator as i32)
                || user.role == (UserRole::Administrator as i32)
            {
                let result = sqlx::query!(
                    "UPDATE tracks SET title=$1 WHERE id=$2 RETURNING *",
                    req.title,
                    req.track_id
                )
                .fetch_one(&self.executor)
                .await
                .map_err(CoursesServiceError::from)?;

                Ok(Response::new(UpdateTrackTitleResponse {
                    track: Some(Track {
                        id: result.id,
                        track_number: result.track_number,
                        title: result.title,
                        audio_path: result.audio_path,
                        document_id: result.document,
                    }),
                }))
            } else {
                Err(tonic::Status::permission_denied(
                    "Only moderators may update tracks.",
                ))
            }
        } else {
            Err(tonic::Status::permission_denied(
                "You must be logged in to update a track.",
            ))
        }
    }

    async fn get_document_bookmarks(
        &self,
        request: tonic::Request<GetDocumentBookmarksRequest>,
    ) -> Result<tonic::Response<GetDocumentBookmarksResponse>, tonic::Status> {
        let req = request.into_inner();
        let bookmarks = sqlx::query!(
            "SELECT * FROM bookmarks WHERE document=$1 LIMIT $2 OFFSET $3;",
            req.document_id,
            req.limit as i64,
            req.offset as i64
        )
        .fetch_all(&self.executor)
        .await
        .map_err(CoursesServiceError::from)?;

        Ok(Response::new(GetDocumentBookmarksResponse {
            total: bookmarks.len() as i32,
            bookmarks: bookmarks
                .into_iter()
                .map(|b| Bookmark {
                    id: b.id,
                    title: b.title,
                    page_id: b.document_page,
                    document_id: b.document,
                })
                .collect(),
        }))
    }

    async fn get_bookmarks_by_ids(
        &self,
        request: tonic::Request<GetBookmarksByIDsRequest>,
    ) -> Result<tonic::Response<GetBookmarksByIDsResponse>, tonic::Status> {
        let req = request.into_inner();
        let bookmarks = sqlx::query!(
            "SELECT * FROM bookmarks WHERE id IN (SELECT * FROM UNNEST($1::int[]));",
            &req.ids
        )
        .fetch_all(&self.executor)
        .await
        .map_err(CoursesServiceError::from)?;

        Ok(Response::new(GetBookmarksByIDsResponse {
            bookmarks: bookmarks
                .into_iter()
                .map(|b| Bookmark {
                    id: b.id,
                    title: b.title,
                    page_id: b.document_page,
                    document_id: b.document,
                })
                .collect(),
        }))
    }

    async fn create_bookmark(
        &self,
        request: tonic::Request<CreateBookmarkRequest>,
    ) -> Result<tonic::Response<CreateBookmarkResponse>, tonic::Status> {
        let req = request.into_inner();
        if let Some(user) = req.active_user {
            if user.role == (UserRole::Moderator as i32)
                || user.role == (UserRole::Administrator as i32)
            {
                let b = (sqlx::query!(
                    "INSERT INTO bookmarks (
                        title,
                        document_page,
                        document
                    ) VALUES ($1, $2, $3) RETURNING *;",
                    req.title,
                    req.page_id,
                    req.document_id
                )
                .fetch_one(&self.executor)
                .await)
                    .map_err(CoursesServiceError::from)?;

                Ok(Response::new(CreateBookmarkResponse {
                    bookmark: Some(Bookmark {
                        id: b.id,
                        title: b.title,
                        page_id: b.document_page,
                        document_id: b.document,
                    }),
                }))
            } else {
                Err(tonic::Status::permission_denied(
                    "Only moderators can create bookmarks.",
                ))
            }
        } else {
            Err(tonic::Status::permission_denied(
                "You must be logged in to create a bookmark.",
            ))
        }
    }

    async fn delete_bookmark(
        &self,
        request: tonic::Request<DeleteBookmarkRequest>,
    ) -> Result<tonic::Response<DeleteBookmarkResponse>, tonic::Status> {
        let req = request.into_inner();
        if let Some(user) = req.active_user {
            if user.role == (UserRole::Moderator as i32)
                || user.role == (UserRole::Administrator as i32)
            {
                (sqlx::query!("DELETE FROM bookmarks WHERE id=$1;", req.bookmark_id)
                    .execute(&self.executor)
                    .await)
                    .map_err(CoursesServiceError::from)?;

                Ok(Response::new(DeleteBookmarkResponse { success: true }))
            } else {
                Err(tonic::Status::permission_denied(
                    "Only moderators can create bookmarks.",
                ))
            }
        } else {
            Err(tonic::Status::permission_denied(
                "You must be logged in to create a bookmark.",
            ))
        }
    }

    async fn get_anchors_by_page_ids(
        &self,
        request: tonic::Request<GetAnchorsByPageIDsRequest>,
    ) -> Result<tonic::Response<GetAnchorsByPageIDsResponse>, tonic::Status> {
        let req = request.into_inner();
        let anchors = sqlx::query!(
            "SELECT * FROM anchors WHERE document_page IN (SELECT * FROM UNNEST($1::int[]));",
            &req.ids
        )
        .fetch_all(&self.executor)
        .await
        .map_err(CoursesServiceError::from)?;

        Ok(Response::new(GetAnchorsByPageIDsResponse {
            anchors: anchors.into_iter().fold(HashMap::new(), |mut acc, cur| {
                acc.entry(cur.document_page)
                    .or_insert(PageAnchors { anchors: vec![] })
                    .anchors
                    .push(Anchor {
                        id: cur.id,
                        title: cur.title.unwrap_or("".to_owned()),
                        track_time: cur.track_time,
                        position_top: cur.position_top,
                        position_left: cur.position_left,
                        page_id: cur.document_page,
                        track_id: cur.track,
                        created_at: cur.created_at.to_rfc3339(),
                        updated_at: cur.updated_at.to_rfc3339(),
                    });
                acc
            }),
        }))
    }

    async fn get_anchors_by_ids(
        &self,
        request: tonic::Request<GetAnchorsByIDsRequest>,
    ) -> Result<tonic::Response<GetAnchorsByIDsResponse>, tonic::Status> {
        let req = request.into_inner();
        let anchors = sqlx::query!(
            "SELECT * FROM anchors WHERE id IN (SELECT * FROM UNNEST($1::int[]));",
            &req.ids
        )
        .fetch_all(&self.executor)
        .await
        .map_err(CoursesServiceError::from)?;

        Ok(Response::new(GetAnchorsByIDsResponse {
            anchors: anchors
                .into_iter()
                .map(|a| Anchor {
                    id: a.id,
                    title: a.title.unwrap_or("".to_owned()),
                    track_time: a.track_time,
                    position_top: a.position_top,
                    position_left: a.position_left,
                    page_id: a.document_page,
                    track_id: a.track,
                    created_at: a.created_at.to_rfc3339(),
                    updated_at: a.updated_at.to_rfc3339(),
                })
                .collect(),
        }))
    }

    async fn get_user_anchors_by_page_ids(
        &self,
        request: tonic::Request<GetUserAnchorsByPageIDsRequest>,
    ) -> Result<tonic::Response<GetUserAnchorsByPageIDsResponse>, tonic::Status> {
        let req = request.into_inner();
        let anchors = sqlx::query!(
            "SELECT * FROM user_anchors WHERE document_page IN (SELECT * FROM UNNEST($1::int[]));",
            &req.ids
        )
        .fetch_all(&self.executor)
        .await
        .map_err(CoursesServiceError::from)?;

        Ok(Response::new(GetUserAnchorsByPageIDsResponse {
            user_anchors: anchors.into_iter().fold(HashMap::new(), |mut acc, cur| {
                acc.entry(cur.document_page)
                    .or_insert(PageUserAnchors { user_anchors: vec![] })
                    .user_anchors
                    .push(UserAnchor {
                        id: cur.id,
                        title: cur.title.unwrap_or("".to_owned()),
                        track_time: cur.track_time,
                        position_top: cur.position_top,
                        position_left: cur.position_left,
                        page_id: cur.document_page,
                        track_id: cur.track,
                        created_at: cur.created_at.to_rfc3339(),
                        updated_at: cur.updated_at.to_rfc3339(),
                        owner: cur.owning_user
                    });
                acc
            }),
        }))
    }

    async fn get_user_anchors_by_ids(
        &self,
        request: tonic::Request<GetUserAnchorsByIDsRequest>,
    ) -> Result<tonic::Response<GetUserAnchorsByIDsResponse>, tonic::Status> {
        let req = request.into_inner();
        let anchors = sqlx::query!(
            "SELECT * FROM user_anchors WHERE id IN (SELECT * FROM UNNEST($1::int[]));",
            &req.ids
        )
        .fetch_all(&self.executor)
        .await
        .map_err(CoursesServiceError::from)?;

        Ok(Response::new(GetUserAnchorsByIDsResponse {
            user_anchors: anchors
                .into_iter()
                .map(|a| UserAnchor {
                    id: a.id,
                    title: a.title.unwrap_or("".to_owned()),
                    track_time: a.track_time,
                    position_top: a.position_top,
                    position_left: a.position_left,
                    page_id: a.document_page,
                    track_id: a.track,
                    created_at: a.created_at.to_rfc3339(),
                    updated_at: a.updated_at.to_rfc3339(),
                    owner: a.owning_user,
                })
                .collect(),
        }))
    }

    async fn create_anchor(
        &self,
        request: tonic::Request<CreateAnchorRequest>,
    ) -> Result<tonic::Response<CreateAnchorResponse>, tonic::Status> {
        let req = request.into_inner();

        let user_role = req
            .active_user
            .map(|u| u.role)
            .unwrap_or(UserRole::Standard as i32);

        if user_role == UserRole::Moderator as i32 || user_role == UserRole::Administrator as i32 {
            let a = (sqlx::query!(
                "INSERT INTO anchors (
                    title,
                    track_time,
                    position_top,
                    position_left,
                    document_page,
                    track,
                    created_at,
                    updated_at
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;",
                req.title,
                req.track_time,
                req.position_top,
                req.position_left,
                req.page_id,
                req.track_id,
                chrono::Utc::now(),
                chrono::Utc::now()
            )
            .fetch_one(&self.executor)
            .await)
                .map_err(CoursesServiceError::from)?;

            Ok(Response::new(CreateAnchorResponse {
                anchor: Some(Anchor {
                    id: a.id,
                    title: a.title.unwrap_or("".to_owned()),
                    track_time: a.track_time,
                    position_top: a.position_top,
                    position_left: a.position_left,
                    page_id: a.document_page,
                    track_id: a.track,
                    created_at: a.created_at.to_rfc3339(),
                    updated_at: a.updated_at.to_rfc3339(),
                }),
            }))
        } else {
            Err(tonic::Status::permission_denied(
                "Only moderators may create anchors.",
            ))
        }
    }

    async fn create_user_anchor(
        &self,
        request: tonic::Request<CreateUserAnchorRequest>,
    ) -> Result<tonic::Response<CreateUserAnchorResponse>, tonic::Status> {
        let req = request.into_inner();

        if let Some(user) = req.active_user {
            let a = (sqlx::query!(
                "INSERT INTO user_anchors (
                    title,
                    track_time,
                    position_top,
                    position_left,
                    document_page,
                    track,
                    owning_user,
                    created_at,
                    updated_at
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;",
                req.title,
                req.track_time,
                req.position_top,
                req.position_left,
                req.page_id,
                req.track_id,
                user.id,
                chrono::Utc::now(),
                chrono::Utc::now()
            )
            .fetch_one(&self.executor)
            .await)
                .map_err(CoursesServiceError::from)?;

            Ok(Response::new(CreateUserAnchorResponse {
                user_anchor: Some(UserAnchor {
                    id: a.id,
                    title: a.title.unwrap_or("".to_owned()),
                    track_time: a.track_time,
                    position_top: a.position_top,
                    position_left: a.position_left,
                    page_id: a.document_page,
                    track_id: a.track,
                    created_at: a.created_at.to_rfc3339(),
                    updated_at: a.updated_at.to_rfc3339(),
                    owner: a.owning_user,
                }),
            }))
        } else {
            Err(tonic::Status::permission_denied(
                "You must be logged in to create a user anchor.",
            ))
        }
    }

    async fn delete_anchor(
        &self,
        request: tonic::Request<DeleteAnchorRequest>,
    ) -> Result<tonic::Response<DeleteAnchorResponse>, tonic::Status> {
        let req = request.into_inner();

        let user_role = req
            .active_user
            .map(|u| u.role)
            .unwrap_or(UserRole::Standard as i32);

        if user_role == UserRole::Moderator as i32 || user_role == UserRole::Administrator as i32 {
            (sqlx::query!("DELETE FROM anchors WHERE id=$1;", req.id)
                .execute(&self.executor)
                .await)
                .map_err(CoursesServiceError::from)?;

            Ok(Response::new(DeleteAnchorResponse { success: true }))
        } else {
            Err(tonic::Status::permission_denied(
                "Only moderators may delete anchors.",
            ))
        }
    }

    async fn delete_user_anchor(
        &self,
        request: tonic::Request<DeleteUserAnchorRequest>,
    ) -> Result<tonic::Response<DeleteUserAnchorResponse>, tonic::Status> {
        let req = request.into_inner();

        if let Some(user) = req.active_user {
            let result =
                (sqlx::query!("SELECT owning_user FROM user_anchors WHERE id=$1;", req.id)
                    .fetch_one(&self.executor)
                    .await)
                    .map_err(CoursesServiceError::from)?;

            if result.owning_user == user.id
                || user.role == UserRole::Moderator as i32
                || user.role == UserRole::Administrator as i32
            {
                (sqlx::query!("DELETE FROM user_anchors WHERE id=$1;", req.id)
                    .execute(&self.executor)
                    .await)
                    .map_err(CoursesServiceError::from)?;

                Ok(Response::new(DeleteUserAnchorResponse { success: true }))
            } else {
                Err(tonic::Status::permission_denied(
                    "You may not delete other users' anchors.",
                ))
            }
        } else {
            Err(tonic::Status::permission_denied(
                "You must be logged in to delete a user anchor.",
            ))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let addr = "[::0]:50051".parse()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    let service = CoursesService::new(pool);

    Server::builder()
        .add_service(CoursesServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
