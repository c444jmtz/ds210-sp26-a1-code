use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV1 {
    model: Llama,
}

impl ChatbotV1 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV1 {
        return ChatbotV1 { model: model };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        
        let mut chat_session: Chat<Llama> = self.model // Initialize a chat session with the model
            .chat() // Start a new chat session
            .with_system_prompt("The assistant will act like a friendly alien"); // Set a system prompt to guide the assistant's behavior


        
        let response = chat_session.add_message(message).await; // Add the user's message to the chat session and await the assistant's response
        
        
        match response { // Handle the response, returning the assistant's reply or an error message
            Ok(text) => text.to_string(),// If the response is successful, convert it to a string and return it
            Err(e) => format!("Sorry, I encountered an error: {}", e),// If there is an error, return a formatted error message
        }
    }
}