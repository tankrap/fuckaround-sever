use crate::{
    db::Database,
    emailer::{self, Emailer},
};
use anyhow::Result;
use lettre::Address;
// use openidconnect::core::{
//     CoreAuthDisplay, CoreClient, CoreErrorResponseType, CoreJsonWebKey,
//     CoreJweContentEncryptionAlgorithm, CoreProviderMetadata,
// CoreRevocableToken, CoreTokenType, };
// use openidconnect::reqwest;
// use openidconnect::{
//     Client, ClientId, ClientSecret, EmptyAdditionalClaims,
// EmptyExtraTokenFields, EndpointMaybeSet,     EndpointNotSet, EndpointSet,
// IdTokenFields, IssuerUrl, OAuth2TokenResponse, RedirectUrl,
//     RevocationErrorResponseType, StandardErrorResponse,
// StandardTokenIntrospectionResponse,     StandardTokenResponse,
//     core::{CoreAuthPrompt, CoreGenderClaim, CoreJwsSigningAlgorithm},
// };
use std::{str::FromStr as _, sync::Arc};

#[derive(Clone)]
pub struct App
{
    pub database: Arc<Database>,
    // pub nats: Arc<async_nats::client::Client>,
    // pub merge_queue: MergeQueue,
    // pub id_gen: Arc<LockMonoUlidGenerator<ULID, MonotonicClock,
    // ThreadRandom>>, pub webauthn: Arc<Webauthn>,
    pub emailer: Option<Emailer>,
    // pub whoami: String, // pub config: CoreConfig,
    // pub modules: Arc<ModuleHandler>,
    // pub tls_config: ServerConfig,
}

impl App
{
    pub async fn init(database: Arc<Database>) -> Result<Self>
    {
        let password = database.get_config::<String>("smtp_password").await;
        let username = database.get_config::<String>("smtp_username").await;
        let server = database.get_config::<String>("smtp_server").await;
        let from_email = database.get_config::<String>("smtp_from_email").await;
        let smtp_config = if let Ok(password) = password
            && let Ok(username) = username
            && let Ok(server) = server
            && let Ok(from_email) = from_email
        {
            Some(emailer::SMTP {
                password,
                username,
                server,
                from: Address::from_str(&from_email)?,
            })
        } else {
            None
        };
        let emailer = Emailer::init(smtp_config);

        Ok(Self { database, emailer })
    }
}
