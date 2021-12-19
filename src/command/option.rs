#![allow(dead_code)]
use std::collections::HashMap;
use serenity::model::guild::PartialMember;
use serenity::model::prelude::application_command::{
    ApplicationCommandInteractionDataOption,
    ApplicationCommandInteractionDataOptionValue
};
use serenity::model::prelude::{PartialChannel, Role, User};

#[derive(Clone)]
pub struct Options {
    options: HashMap<String, Option<ApplicationCommandInteractionDataOptionValue>>,
}

impl Options {
    pub fn new(options_vec: Vec<ApplicationCommandInteractionDataOption>) -> Self {
        let mut options = HashMap::new();

        for option in options_vec {
            if let Some(_) = options.insert(option.name.clone(), option.resolved) {
                panic!("Options cannot have duplicating names: {}", option.name);
            }
        }

        Self { options }
    }
    pub async fn read(&mut self, name: &'static str) -> Option<ApplicationCommandInteractionDataOptionValue> {
        self.options
            .remove(name)
            .unwrap_or_else(|| panic!("No option with name {} found.", name))
    }
    pub async fn read_string(&mut self, name: &'static str) -> Option<String> {
        match self.read(name).await {
            Some(ApplicationCommandInteractionDataOptionValue::String(string)) => Some(string),
            None => None,
            _ => panic!("Type mismatch for channel named {}", name),
        }
    }
    pub async fn read_integer(&mut self, name: &'static str) -> Option<i64> {
        match self.read(name).await {
            Some(ApplicationCommandInteractionDataOptionValue::Integer(integer)) => Some(integer),
            None => None,
            _ => panic!("Type mismatch for channel named {}", name),
        }
    }
    pub async fn read_boolean(&mut self, name: &'static str) -> Option<bool> {
        match self.read(name).await {
            Some(ApplicationCommandInteractionDataOptionValue::Boolean(boolean)) => Some(boolean),
            None => None,
            _ => panic!("Type mismatch for channel named {}", name),
        }
    }
    pub async fn read_user(&mut self, name: &'static str) -> Option<(User, Option<PartialMember>)> {
        match self.read(name).await {
            Some(ApplicationCommandInteractionDataOptionValue::User(user, member))
                => Some((user, member)),
            None => None,
            _ => panic!("Type mismatch for channel named {}", name),
        }
    }
    pub async fn read_channel(&mut self, name: &'static str) -> Option<PartialChannel> {
        match self.read(name).await {
            Some(ApplicationCommandInteractionDataOptionValue::Channel(channel)) => Some(channel),
            None => None,
            _ => panic!("Type mismatch for channel named {}", name),
        }
    }
    pub async fn read_role(&mut self, name: &'static str) -> Option<Role> {
        match self.read(name).await {
            Some(ApplicationCommandInteractionDataOptionValue::Role(role)) => Some(role),
            None => None,
            _ => panic!("Type mismatch for channel named {}", name),
        }
    }
    pub async fn read_number(&mut self, name: &'static str) -> Option<f64> {
        match self.read(name).await {
            Some(ApplicationCommandInteractionDataOptionValue::Number(number)) => Some(number),
            None => None,
            _ => panic!("Type mismatch for channel named {}", name),
        }
    }
}