use crate::command::Command;

pub trait Template: Sized + 'static {
    fn run(&self, command: Command) -> Command;
    fn build(self) -> Box<dyn FnMut(Command) -> Command> {
        Box::new(move |command| self.run(command))
    }
}