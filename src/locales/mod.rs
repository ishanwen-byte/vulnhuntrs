pub mod en;
pub mod ja;
pub mod cn;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    Japanese,
    English,
    Chinese,
}

impl Language {
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "cn" | "chinese" => Language::Chinese,
            "ja" | "japanese" => Language::Japanese,
            "en" | "english" => Language::English,
            _ => Language::English, // Default to English
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Language::Chinese => "cn",
            Language::Japanese => "ja",
            Language::English => "en",
        }
    }
}

pub struct LanguageConfig {
    pub language: Language,
}

impl LanguageConfig {
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    pub fn get_message(&self, key: &str) -> &str {
        let messages = get_messages(&self.language);
        messages.get(key).unwrap_or(&"Message not found")
    }

    pub fn get_analysis_prompt(&self) -> &str {
        match self.language {
            Language::Chinese => "请务必使用中文回答",
            Language::Japanese => "必ず日本語で応答してください",
            Language::English => "Please respond in English",
        }
    }

    pub fn get_response_language_instruction(&self) -> &str {
        get_response_language_instruction(&self.language)
    }
}

pub fn get_messages(lang: &Language) -> HashMap<&'static str, &'static str> {
    match lang {
        Language::Chinese => cn::get_messages(),
        Language::Japanese => ja::get_messages(),
        Language::English => en::get_messages(),
    }
}

pub fn get_sys_prompt_template(lang: &Language) -> &'static str {
    match lang {
        Language::Chinese => cn::SYS_PROMPT_TEMPLATE,
        Language::Japanese => ja::SYS_PROMPT_TEMPLATE,
        Language::English => en::SYS_PROMPT_TEMPLATE,
    }
}

pub fn get_initial_analysis_prompt_template(lang: &Language) -> &'static str {
    match lang {
        Language::Chinese => cn::INITIAL_ANALYSIS_PROMPT_TEMPLATE,
        Language::Japanese => ja::INITIAL_ANALYSIS_PROMPT_TEMPLATE,
        Language::English => en::INITIAL_ANALYSIS_PROMPT_TEMPLATE,
    }
}

pub fn get_analysis_approach_template(lang: &Language) -> &'static str {
    match lang {
        Language::Chinese => cn::ANALYSIS_APPROACH_TEMPLATE,
        Language::Japanese => ja::ANALYSIS_APPROACH_TEMPLATE,
        Language::English => en::ANALYSIS_APPROACH_TEMPLATE,
    }
}

pub fn get_guidelines_template(lang: &Language) -> &'static str {
    match lang {
        Language::Chinese => cn::GUIDELINES_TEMPLATE,
        Language::Japanese => ja::GUIDELINES_TEMPLATE,
        Language::English => en::GUIDELINES_TEMPLATE,
    }
}

pub fn get_evaluator_prompt_template(lang: &Language) -> &'static str {
    match lang {
        Language::Chinese => cn::EVALUATOR_PROMPT_TEMPLATE,
        Language::Japanese => ja::EVALUATOR_PROMPT_TEMPLATE,
        Language::English => en::EVALUATOR_PROMPT_TEMPLATE,
    }
}

pub fn get_response_language_instruction(lang: &Language) -> &'static str {
    match lang {
        Language::Chinese => cn::RESPONSE_LANGUAGE_INSTRUCTION,
        Language::Japanese => ja::RESPONSE_LANGUAGE_INSTRUCTION,
        Language::English => en::RESPONSE_LANGUAGE_INSTRUCTION,
    }
}
