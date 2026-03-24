use kalosm::language::*;
use crate::solution::file_library;
use std::collections::HashMap;

pub struct ChatbotV4 {
    model: Llama,
    user_sessions: HashMap<String, Chat<Llama>>,  // In-memory cache of user sessions
}

impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        return ChatbotV4 {
            model: model,
            user_sessions: HashMap::new(),
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = format!("{}.txt", username);

        // Get the chat session for the user, either from memory or from file
        // or create a new one if it doesn't exist
        let chat_session = if let Some(session) = self.user_sessions.get_mut(&username) {
            // Found in memory - use existing session
            session
        } else {
            // Not in memory - try to load from file
            match file_library::load_chat_session_from_file(&filename) {
                Some(loaded_session) => {
                    // Found in file - create a Chat from the loaded session
                    let chat = self.model
                        .chat()
                        .with_session(loaded_session);
                    self.user_sessions.insert(username.clone(), chat);
                    self.user_sessions.get_mut(&username).unwrap()
                }
                None => {
                    // No existing session - create a brand new one
                    let new_session = self.model
                        .chat()
                        .with_system_prompt("The assistant will act like a pirate");
                    self.user_sessions.insert(username.clone(), new_session);
                    self.user_sessions.get_mut(&username).unwrap()
                }
            }
        };
        
        // Add the user's message and get the assistant's response
        match chat_session.add_message(message).await {
            Ok(text) => {
                let response = text.to_string();
                
                // Save the updated session to file
                if let Err(e) = self.save_chat_session_to_file(&username) {
                    eprintln!("Error saving session for {}: {}", username, e);
                }
                
                response
            }
            Err(e) => format!("Sorry, I encountered an error: {}", e),
        }
    }

    pub fn save_chat_session_to_file(&self, username: &str) -> Result<(), Box<dyn std::error::Error + '_>> {
        let filename = format!("{}.txt", username);
        
        // Get the session from memory
        if let Some(chat) = self.user_sessions.get(username) {
            // Get the LlamaChatSession from the Chat
            let session_data = chat.session()?;
            // Save using file_library function
            file_library::save_chat_session_to_file(&filename, &session_data)?;
        }
        
        Ok(())
    }

    pub fn get_history(&self, username: String) -> Vec<String> {
        let filename = format!("{}.txt", username);

        match file_library::load_chat_session_from_file(&filename) {
            None => {
                return Vec::new();
            },
            Some(session) => {
                // session is LlamaChatSession, which has history() directly
                let history = session.history();
                history.iter().map(|msg| format!("{:?}", msg)).collect()
            }
        }
    }
}
