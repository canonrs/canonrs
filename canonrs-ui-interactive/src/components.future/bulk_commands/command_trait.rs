pub trait Command {
    fn execute(&self);
    fn undo(&self);
    fn description(&self) -> String;
}
