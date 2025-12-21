use bzd_lib::error::Error;
use futures_lite::StreamExt as _;
use tracing::{error, info};

use crate::app::state::AppState;

pub async fn messages(state: &AppState) -> Result<(), Error> {
    let AppState { js, settings, .. } = state.clone();

    let consumer = messages::consumer(&js, &settings).await?;
    let mut messages = consumer.messages().await?;

    while let Some(message) = messages.next().await {
        if let Err(err) = messages::handler(state, message?).await {
            error!("{}", err);
        }
    }

    Ok(())
}

mod messages {
    use async_nats::jetstream::{
        self,
        consumer::{Consumer, pull::Config},
    };
    use bzd_lib::error::Error;
    use tracing::info;
    use uuid::Uuid;

    use crate::app::{error::AppError, feeds::service, settings::AppSettings, state::AppState};

    pub async fn consumer(
        js: &jetstream::Context,
        settings: &AppSettings,
    ) -> Result<Consumer<Config>, Error> {
        Ok(js
            .create_consumer_on_stream(
                Config {
                    durable_name: Some(settings.feeds.messaging.message.consumer.clone()),
                    filter_subjects: settings.feeds.messaging.message.subjects.clone(),
                    ..Default::default()
                },
                settings.nats.stream.clone(),
            )
            .await?)
    }

    pub async fn handler(state: &AppState, message: jetstream::Message) -> Result<(), AppError> {
        let AppState { settings, db, js } = state;

        info!("MESS EVENT");

        // dbg!(&message);
        // dbg!(&message.payload);

        // let req = service::create_message::Request {
        //     message_id: Uuid::now_v7(),
        //     topic_ids: vec![Uuid::now_v7()],
        // };
        // service::create_message(db, req).await?;

        // service::messaging_stream(state, message.clone().try_into()?).await?;

        message.ack().await?;
        Ok(())
    }
}

pub async fn users_topics(state: &AppState) -> Result<(), Error> {
    let AppState { js, settings, .. } = state;

    let consumer = users_topics::consumer(&js, &settings).await?;
    dbg!(&consumer);
    let mut messages = consumer.messages().await?;

    while let Some(message) = messages.next().await {
        if let Err(err) = users_topics::handler(state, message?).await {
            error!("{}", err);
        }
    }

    Ok(())
}

mod users_topics {
    use async_nats::jetstream::{
        self,
        consumer::{Consumer, pull::Config},
    };
    use bzd_lib::error::Error;
    use chrono::Utc;
    use prost::Message as _;

    use crate::app::{
        error::AppError,
        feeds::service::{self, create_topic_user::Request},
        settings::AppSettings,
        state::AppState,
    };

    pub async fn consumer(
        js: &jetstream::Context,
        settings: &AppSettings,
    ) -> Result<Consumer<Config>, Error> {
        Ok(js
            .create_consumer_on_stream(
                Config {
                    durable_name: Some(settings.feeds.messaging.topic_user.consumer.clone()),
                    filter_subjects: settings.feeds.messaging.topic_user.subjects.clone(),
                    ..Default::default()
                },
                settings.nats.stream.clone(),
            )
            .await?)
    }

    pub async fn handler(state: &AppState, message: jetstream::Message) -> Result<(), AppError> {
        let AppState { settings, db, js } = state;

        if let Some(headers) = message.headers.as_ref() {
            let tp = headers.get("ce_type");
        }

        service::create_topic_user(db, message.clone().try_into()?).await?;

        message.ack().await?;

        Ok(())
    }

    impl TryFrom<jetstream::Message> for Request {
        type Error = AppError;

        fn try_from(message: jetstream::Message) -> Result<Self, Self::Error> {
            let message = bzd_messages_api::events::TopicUser::decode(message.payload.clone())?;

            Ok(Self {
                topic_user_id: message.topic_user_id().parse()?,
                topic_id: message.topic_id().parse()?,
                user_id: message.user_id().parse()?,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            })
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use bzd_lib::error::Error;

//     use crate::app::feeds::messaging::users_topics;

//     #[tokio::test]
//     async fn users_topics_handler() -> Result<(), Error> {
//         users_topics::handler(state, message).await?;

//         Ok(())
//     }
// }
