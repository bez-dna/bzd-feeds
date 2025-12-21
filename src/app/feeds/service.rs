use bzd_lib::error::Error;
use sea_orm::DbConn;

use crate::app::{error::AppError, feeds::repo};

pub async fn create_message(db: &DbConn, req: create_message::Request) -> Result<(), Error> {
    let task = repo::task::Model::new(req.into());
    let task = repo::create_task(db, task).await?;

    Ok(())
}

pub mod create_message {
    use uuid::Uuid;

    use crate::app::feeds::repo::task::{CreateMessage, Payload};

    pub struct Request {
        pub message_id: Uuid,
        pub topic_ids: Vec<Uuid>,
    }

    impl From<Request> for Payload {
        fn from(req: Request) -> Self {
            Self::CreateMessage(CreateMessage {
                message_id: req.message_id,
                topic_ids: req.topic_ids,
                last_topic_user_id: None,
            })
        }
    }

    impl From<CreateMessage> for Request {
        fn from(payload: CreateMessage) -> Self {
            Self {
                message_id: payload.message_id,
                topic_ids: payload.topic_ids,
            }
        }
    }
}

pub async fn create_entries_from_message(
    db: &DbConn,
    req: create_entries_from_message::Request,
) -> Result<(), Error> {
    Ok(())
}

pub mod create_entries_from_message {
    use uuid::Uuid;

    use crate::app::feeds::repo::task::CreateMessage;

    pub struct Request {
        pub message_id: Uuid,
        pub topic_ids: Vec<Uuid>,
    }

    impl From<CreateMessage> for Request {
        fn from(payload: CreateMessage) -> Self {
            Self {
                message_id: payload.message_id,
                topic_ids: payload.topic_ids,
            }
        }
    }
}

pub async fn create_topic_user(
    db: &DbConn,
    req: create_topic_user::Request,
) -> Result<(), AppError> {
    let topic_user: repo::topic_user::Model = req.into();
    repo::create_topic_user(db, topic_user).await?;

    Ok(())
}

pub mod create_topic_user {
    use chrono::NaiveDateTime;
    use uuid::Uuid;

    use crate::app::feeds::repo;

    pub struct Request {
        pub topic_user_id: Uuid,
        pub topic_id: Uuid,
        pub user_id: Uuid,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl From<Request> for repo::topic_user::Model {
        fn from(req: Request) -> Self {
            Self {
                topic_user_id: req.topic_user_id,
                user_id: req.user_id,
                topic_id: req.topic_id,
                created_at: req.created_at,
                updated_at: req.updated_at,
            }
        }
    }
}
