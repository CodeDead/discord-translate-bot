use crate::models::google_translate_error::GoogleTranslateError;
use log::{error, info};

pub struct GoogleTranslateService {
    client: reqwest::Client,
    token: String,
}

impl GoogleTranslateService {
    /// Create a new GoogleTranslateService instance
    ///
    /// # Arguments
    ///
    /// * `token` - The Google Translate API token
    ///
    /// # Returns
    ///
    /// A new GoogleTranslateService instance
    pub fn new(token: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            token: String::from(token),
        }
    }

    /// Translate text
    ///
    /// # Arguments
    ///
    /// * `text` - The text to translate
    /// * `target` - The target language
    ///
    /// # Returns
    ///
    /// A `Result` containing the translated text or a `GoogleTranslateError` instance
    pub async fn translate(
        &self,
        text: &str,
        target: &str,
    ) -> Result<String, GoogleTranslateError> {
        info!("Translating text \"{}\" to target \"{}\"", text, target);

        let url = format!(
            "https://translation.googleapis.com/language/translate/v2?key={}&q={}&target={}",
            self.token, text, target
        );

        let response = match self.client.get(&url).send().await {
            Ok(response) => response,
            Err(err) => {
                error!("Failed to send request to Google Translate: {:?}", err);
                return Err(GoogleTranslateError::new(&format!(
                    "Failed to send request to Google Translate: {}",
                    err
                )));
            }
        };

        let response = match response.json::<serde_json::Value>().await {
            Ok(response) => response,
            Err(err) => {
                error!("Failed to parse response from Google Translate: {:?}", err);
                return Err(GoogleTranslateError::new(&format!(
                    "Failed to parse response from Google Translate: {}",
                    err
                )));
            }
        };

        let translation = response["data"]["translations"][0]["translatedText"]
            .as_str()
            .ok_or_else(|| {
                error!("Failed to get translated text");
                GoogleTranslateError::new("Failed to get translated text")
            })?;

        Ok(translation.to_string())
    }
}
