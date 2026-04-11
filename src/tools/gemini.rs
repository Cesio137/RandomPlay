use genai::chat::{ChatMessage, ChatRequest};
use genai::Client;
use once_cell::sync::Lazy;
use std::error::Error;

const MODEL_GEMINI: &str = "gemini-3-flash-preview";

pub static GEMINI: Lazy<Client> = Lazy::new(|| Client::default());

pub async fn get_text(prompt: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    _ = GEMINI.resolve_service_target(MODEL_GEMINI).await?;

    let chat_req = ChatRequest::new(vec![
        //ChatMessage::system(""),
        ChatMessage::user(prompt),
    ]);

    let chat_res = GEMINI.exec_chat(MODEL_GEMINI, chat_req, None).await?;

    let texts = &chat_res.into_texts();
    Ok(texts.join("\n"))
}
