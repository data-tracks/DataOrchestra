use serde::{Deserialize, Serialize};

use crate::{core::{adapters::TmuxBuilder, types::{DataTypes, Executables, Script, VolatileData}}, shared::ToInternal};

use super::location::Location;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ExtExecutables {
    Script(ExtScript),
    Tmux(ExtTmux)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExtTmux {
    #[serde(default)]
    pub location: Location,
    pub name: Option<String>,
    pub path: String,
    #[serde(flatten)]
    pub tmux: TmuxBuilder 
}

impl ToInternal<(Script, DataTypes)> for ExtTmux {
    fn to_internal(mut self) -> (Script, DataTypes) {
        let script = Script { name: self.name.clone(), path: self.path.clone() };

        let data = VolatileData { name: self.name, content: self.tmux.build(), file: self.path.into()  };

        (script, DataTypes::VolatileDockerData(data))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ExtScript {
    pub location: Location,
    pub name: Option<String>,
    pub path: String
}

impl ToInternal<Script> for ExtScript {
    fn to_internal(self) -> Script {
        Script { name: self.name, path: self.path }
    }
}

impl ToInternal<(Executables, Option<DataTypes>)> for ExtExecutables {
    fn to_internal(self) -> (Executables, Option<DataTypes>) {
        match self {
            ExtExecutables::Tmux(tmux) => { 
                let (script, data) = tmux.to_internal();
                (Executables::Script(script), Some(data))
            }
            ExtExecutables::Script(script) => {
                let script = script.to_internal();
                (Executables::Script(script), None)
            }
        } 
    }
}
