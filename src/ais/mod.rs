// region:  --- Modules

pub mod asst;

use async_openai::{config::OpenAIConfig, Client};

use crate::Result;

// region: --- Client

const ENV_OPENAI_API_KEY: &str = "OPENAI_API_KEY";

pub type OaClient = Client<OpenAIConfig>;

pub fn new_oa_client() -> Result<OaClient> {
    if std::env::var(ENV_OPENAI_API_KEY).is_ok() {
        Ok(Client::new())
    } else {
        println!("No {ENV_OPENAI_API_KEY} env variable. Please set it.");
        Err("No openai api key in env".into())
    }
}

// endregion: --- Modules

