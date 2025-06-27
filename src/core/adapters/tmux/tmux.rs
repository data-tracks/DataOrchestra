use derive_builder::Builder;

#[derive(Debug, Builder)]
#[builder(build_fn(name = "build_internal", private))]
pub struct Tmux {
    #[builder(default = "true", setter(custom))]
    bash: bool,
    #[builder(default = "false", setter(custom))]
    sh: bool,
    #[builder(setter(into))]
    pub session: String,
    #[builder(setter(custom))]
    pub commands: Vec<String>
}

impl TmuxBuilder {
    pub fn bash(&mut self) -> &mut Self {
        self.bash.get_or_insert(true);
        self
    }

    pub fn sh(&mut self) -> &mut Self {
        self.sh.get_or_insert(true);
        self
    }

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

        let shell: String;
        if tmux.sh {
            shell = "#!/usr/bin/sh".to_string();
        }
        else if tmux.bash {
            shell = "#!/usr/bin/bash".to_string();
        }
        else {
            panic!("Please set shell for tmux session");
        }

        let session = format!("session={}", tmux.session);
        let commands = tmux.commands.join("\n");

        format!("{}\n{}\n{}", shell, session, commands) 
    }
}

#[cfg(test)]
mod tests {
    use super::TmuxBuilder;

    #[test]
    #[should_panic]
    pub fn no_session() {
        let _ = TmuxBuilder::default()
            .build();
    } 

    #[test]
    #[should_panic]
    pub fn commands_no_session() {
        let _ = TmuxBuilder::default()
            .command("cargo run")
            .build();
    } 

    #[test]
    #[should_panic]
    pub fn no_commands() {
        let _ = TmuxBuilder::default()
            .session("TestSession")
            .build();
    }

    #[test]
    pub fn tmux_bash_command_create() {
        let tmux = TmuxBuilder::default()
            .session("TestSession")
            .command("cargo run")
            .build();

        assert_eq!(tmux, "#!/usr/bin/bash\nsession=TestSession\ntmux send-keys -t $session \"cargo run\" C-m");
    }

    #[test]
    pub fn tmux_sh_command_create() {
        let tmux = TmuxBuilder::default()
            .sh()
            .session("TestSession")
            .command("cargo run")
            .build();

        assert_eq!(tmux, "#!/usr/bin/sh\nsession=TestSession\ntmux send-keys -t $session \"cargo run\" C-m");
    }

    #[test]
    pub fn tmux_multiple_command_create() {
        let tmux = TmuxBuilder::default()
            .session("TestSession")
            .command("cargo test")
            .command("cargo run")
            .build();

        assert_eq!(tmux, "#!/usr/bin/bash\nsession=TestSession\ntmux send-keys -t $session \"cargo test\" C-m\ntmux send-keys -t $session \"cargo run\" C-m");
    }
}
