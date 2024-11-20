use openai_api_rust::{
    chat::{ChatApi, ChatBody},
    Auth, Message, OpenAI, Role,
};

struct AiHandler {
    pub initial_prompt: String,
}

impl AiHandler {
    fn new(initial_prompt: String) -> Self {
        Self { initial_prompt }
    }

    fn raw_request(&self, prompt: &str) -> Option<String> {
        let openai = OpenAI::new(Auth::from_env().unwrap(), "https://api.openai.com/v1/");

        let body = ChatBody {
            model: "gpt-3.5-turbo".to_string(),
            max_tokens: None,
            temperature: Some(0_f32),
            top_p: Some(0_f32),
            n: Some(2),
            stream: Some(false),
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
            messages: vec![
                Message {
                    role: Role::System,
                    content: self.initial_prompt.clone(),
                },
                Message {
                    role: Role::User,
                    content: prompt.to_string(),
                },
            ],
        };

        let response = match openai.chat_completion_create(&body) {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Error: {:?}", e);
                return None;
            }
        };

        let response = match response.choices.get(0) {
            Some(response) => response,
            None => return None,
        };

        Some(response.message.clone().unwrap().content)
    }
}
