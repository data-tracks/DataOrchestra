use derive_builder::Builder;

#[derive(Debug, Builder)]
#[builder(build_fn(name = "build_internal", private))]
pub struct Tmux {
    #[builder(setter(into))]
    pub session: String,
    #[builder(setter(custom), default)]
    pub commands: Vec<String>
}

impl TmuxBuilder {
    pub fn command(&mut self, command: impl Into<String>) -> &mut Self {
        let vec = self.commands.get_or_insert_default();
        let command = format!("tmux send-keys -t $session \"{}\" C-m", command.into());
        vec.push(command);
        self
    }

    pub fn build(&mut self) -> String {
        let tmux = self
            .build_internal()
            .expect("Unable to build tmux");

        let session = format!("session={}", tmux.session);
        let commands = tmux.commands.join("\n");

        format!("{}\n{}", session, commands) 
    }
}
