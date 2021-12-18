use std::collections::HashMap;
use std::sync::Arc;
use sqlx::SqlitePool;
use serenity::{
    http::Http,
    builder::{CreateApplicationCommands},
    model::{
        id::GuildId,
        interactions::InteractionResponseType,
        prelude::application_command::{ApplicationCommand, ApplicationCommandInteraction}
    },
    prelude::Context
};
use crate::Caches;
use crate::command::command::CommandContext;
use crate::command::option::Options;
use crate::command::Template;
use crate::config::Config;
use super::command::{Command, Response, CommandHolder, ResponseFnHolder};

pub struct CommandsHandler {
    config: Arc<Config>,
    db: SqlitePool,
    caches: Caches,
    registry: HashMap<String, CommandHolder>,
    responses: HashMap<String, ResponseFnHolder>,
}

impl CommandsHandler {
    pub fn new(config: Arc<Config>, db: SqlitePool, caches: Caches) -> Self {
        Self {
            config,
            db,
            caches,
            registry: HashMap::default(),
            responses: HashMap::default(),
        }
    }
    pub fn add_command<T: ToString>(mut self, name: T, command_fn: impl FnOnce(Command) -> Command) -> Self {
        let name = name.to_string();
        let (command, response) =
            Command::fn_to_parts(name.clone(), command_fn, self.config.clone(), self.db.clone());
        self.registry.insert(name.clone(), command);
        self.responses.insert(name, response);

        self
    }
    pub fn add_template_command<T, R>(
        self,
        name: T,
        template_fn: impl FnOnce() -> R,
    ) -> Self
    where
        T: ToString,
        R: Template,
    {
        self.add_command(name, template_fn().build())
    }
    pub fn register_commands(&self, commands: &mut CreateApplicationCommands) {
        for command in self.registry.values() {
            commands.create_application_command(command.create_fn());
        }
    }
    pub async fn register_permissions(
        &self,
        guild: &mut GuildId,
        http: impl AsRef<Http>,
        commands: Vec<ApplicationCommand>
    ) {
        let commands_with_permission = commands
            .iter()
            .zip(self.registry.values())
            .filter(|(_, x)| x.permission_fn.is_some());

        for (app, command) in commands_with_permission {
            guild
                .create_application_command_permission(&http, app.id, |permissions| {
                    let f = command.permission_fn.as_ref()
                        .expect("Critical Error. Permissions state is corrupt");
                    f(permissions)
                })
                .await
                .unwrap();
        }
    }
    pub async fn handle_response(
        &self,
        command: &ApplicationCommandInteraction,
        ctx: &Context,
    ) {
        let f = self.responses
            .get(&command.data.name)
            .expect("Critical Error. Mismatching application command name and response name")
            .response_fn
            .as_ref();

        let context = CommandContext {
            http: ctx.http.clone(),
            config: self.config.clone(),
            db: self.db.clone(),
            caches: self.caches.clone(),
            options: Options::new(command.data.options.clone()),
            guild_id: command.guild_id.clone(),
            channel_id: command.channel_id.clone(),
            member: command.member.clone(),
            user: command.user.clone(),
        };

        let response_fn: Response = f(context).await;

        if let Err(e) = {
            command.create_interaction_response(&ctx.http, |response| {
                match response_fn {
                    Response::Message(f) => response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(f),
                    Response::DeferredMessage(f) => response
                        .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                        .interaction_response_data(f),
                }
            }).await
        } {
            println!("Error while interaction response: {}", e);
        }
    }
}