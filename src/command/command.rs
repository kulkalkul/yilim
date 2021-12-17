#![allow(dead_code)]

use std::future::Future;
use std::sync::Arc;
use futures::future::BoxFuture;
use sqlx::SqlitePool;
use serenity::builder::{
    CreateApplicationCommand,
    CreateApplicationCommandOption,
    CreateApplicationCommandPermissionsData,
    CreateInteractionResponseData
};
use serenity::http::Http;
use serenity::model::id::ChannelId;
use serenity::model::prelude::{GuildId, Member, User};
use crate::Caches;
use crate::command::option::Options;
use crate::config::Config;

pub struct CommandContext {
    pub http: Arc<Http>,
    pub config: Arc<Config>,
    pub db: SqlitePool,
    pub caches: Caches,
    pub options: Options,
    pub guild_id: Option<GuildId>,
    pub channel_id: ChannelId,
    pub member: Option<Member>,
    pub user: User,
}

pub struct Command {
    pub config: Arc<Config>,
    pub db: SqlitePool,
    create_application_command: CreateApplicationCommand,
    response_fn: Option<Box<dyn Fn(CommandContext) -> BoxFuture<'static, Response> + Sync + Send>>,
    permission_fn: Option<Box<dyn Fn(&mut CreateApplicationCommandPermissionsData) ->
        &mut CreateApplicationCommandPermissionsData + Sync + Send>>,
}

impl Command {
    pub fn new(config: Arc<Config>, db: SqlitePool) -> Self {
        Self {
            config,
            db,
            create_application_command: CreateApplicationCommand::default(),
            response_fn: None,
            permission_fn: None,
        }
    }
    pub fn fn_to_parts(
        name: String,
        command_fn: impl FnOnce(Command) -> Command,
        config: Arc<Config>,
        db: SqlitePool,
    ) -> (CommandHolder, ResponseFnHolder)
    {
        (command_fn)(Command::new(config, db)).to_parts(name)
    }

    pub fn default_permission(&mut self, default_permission: bool) -> &mut Self {
        self.create_application_command.default_permission(default_permission);
        self
    }
    pub fn description<D: ToString>(&mut self, description: D) -> &mut Self {
        self.create_application_command.description(description);
        self
    }
    pub fn create_option<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut CreateApplicationCommandOption) -> &mut CreateApplicationCommandOption,
    {
        self.create_application_command.create_option(f);
        self
    }
    pub fn add_option(&mut self, option: CreateApplicationCommandOption) -> &mut Self {
        self.create_application_command.add_option(option);
        self
    }
    pub fn set_options(&mut self, options: Vec<CreateApplicationCommandOption>) -> &mut Self {
        self.create_application_command.set_options(options);
        self
    }
    pub fn set_response<Fut>(
        &mut self,
        response_fn: impl Fn(CommandContext) -> Fut + Sync + Send + 'static
    ) -> &mut Self
    where
        Fut: Future<Output = Response> + 'static + Send,
    {
        self.response_fn = Some(Box::new(move |ctx| Box::pin(response_fn(ctx))));
        self
    }
    pub fn set_permissions(
        &mut self,
        permissions_fn: impl Fn(&mut CreateApplicationCommandPermissionsData) ->
            &mut CreateApplicationCommandPermissionsData + Sync + Send + 'static,
    ) -> &mut Self {
        self.permission_fn = Some(Box::new(permissions_fn));
        self
    }
    fn to_parts(self, name: String) -> (CommandHolder, ResponseFnHolder) {
        (
            CommandHolder {
                name: name.clone(),
                permission_fn: self.permission_fn,
                create_application_command: self.create_application_command,
            },
            ResponseFnHolder {
                name: name.clone(),
                response_fn: self.response_fn
                    .unwrap_or_else(|| panic!("Command {} is missing response!", name)),
            },
        )
    }
}

pub struct CommandHolder {
    pub name: String,
    pub permission_fn: Option<Box<dyn Fn(&mut CreateApplicationCommandPermissionsData) ->
        &mut CreateApplicationCommandPermissionsData + Sync + Send>>,
    create_application_command: CreateApplicationCommand,
}

impl CommandHolder {
    pub fn create_fn(&self) -> impl FnOnce(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand + '_ {
        let name = self.name.clone();

        |command| {
            *command = self.create_application_command.clone();
            command.name(name);

            command
        }
    }
}

pub struct ResponseFnHolder {
    pub name: String,
    pub response_fn: Box<dyn Fn(CommandContext) -> BoxFuture<'static, Response> + Sync + Send>,
}

pub enum Response {
    Message(Box<dyn FnOnce(&mut CreateInteractionResponseData) -> &mut CreateInteractionResponseData + Send>),
    DeferredMessage(Box<dyn FnOnce(&mut CreateInteractionResponseData) -> &mut CreateInteractionResponseData + Send>),
}

impl Response {
    pub fn message(
        interaction_fn: impl FnOnce(&mut CreateInteractionResponseData) ->
            &mut CreateInteractionResponseData + Send + 'static
    ) -> Self {
        Self::Message(Box::new(interaction_fn))
    }
    pub fn deferred_message(
        interaction_fn: impl FnOnce(&mut CreateInteractionResponseData) ->
            &mut CreateInteractionResponseData + Send + 'static
    ) -> Self {
        Self::DeferredMessage(Box::new(interaction_fn))
    }
}