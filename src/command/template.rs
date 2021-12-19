use std::sync::Arc;
use sqlx::SqlitePool;
use crate::{Caches, Config};
use crate::command::Command;

#[derive(Clone)]
pub struct TemplateContext {
    pub config: Arc<Config>,
    pub db: SqlitePool,
    pub caches: Caches,
}

pub trait Template: Sized + 'static {
    fn run(&self, command: Command) -> Command;
    fn build(self) -> Box<dyn FnMut(Command) -> Command> {
        Box::new(move |command| self.run(command))
    }
}