#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use derive_builder::Builder;
type Option = ();
type Some = ();
type None = ();
type Result = ();
type Box = ();
pub struct Command {
    executable: String,
}
pub struct CommandBuilder {
    executable: std::option::Option<String>,
}
impl Command {
    pub fn builder() -> CommandBuilder {
        CommandBuilder {
            executable: std::option::Option::None,
        }
    }
}
impl CommandBuilder {
    fn executable(&mut self, executable: String) -> &mut Self {
        self.executable = std::option::Option::Some(executable);
        self
    }
    pub fn build(
        &mut self,
    ) -> std::result::Result<Command, std::boxed::Box<dyn std::error::Error>> {
        std::result::Result::Ok(Command {
            executable: self.executable.clone().unwrap(),
        })
    }
}
fn main() {}
