use crate::handlers::push_message::MessagePayload;
use crate::providers::PushProvider;
use a2::NotificationBuilder;
use async_trait::async_trait;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct ApnsProvider {
    client: a2::Client,
}

impl ApnsProvider {
    pub fn new_cert<R>(
        cert: &mut R,
        password: String,
        endpoint: a2::Endpoint,
    ) -> crate::error::Result<Self>
    where
        R: Read,
    {
        Ok(ApnsProvider {
            client: a2::Client::certificate(cert, password.as_str(), endpoint)?,
        })
    }

    pub fn new_token<R>(
        pem: &mut R,
        key_id: String,
        team_id: String,
        endpoint: a2::Endpoint,
    ) -> crate::error::Result<Self>
    where
        R: Read,
    {
        Ok(ApnsProvider {
            client: a2::Client::token(pem, key_id, team_id, endpoint)?,
        })
    }
}

#[async_trait]
impl PushProvider for ApnsProvider {
    async fn send_notification(
        &mut self,
        token: String,
        payload: MessagePayload,
    ) -> crate::error::Result<()> {
        let opt = a2::NotificationOptions::default();

        // TODO set title
        let notification =
            a2::PlainNotificationBuilder::new(&payload.description).build(token.as_str(), opt);

        let _ = self.client.send(notification).await?;

        Ok(())
    }
}
