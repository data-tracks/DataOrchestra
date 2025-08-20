use serde::{Deserialize, Serialize};

use crate::{core::{adapters::TmuxBuilder, types::{DataTypes, Executables, Script, VolatileData}}, shared::ToInternal};
use crate::core::types::DataTypes::{VolatileDockerData, VolatileNodeData};
use crate::interface::data::{ExtData, ExtDataTypes, ExtVolatile};
use super::location::Location;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ExtExecutables {
    Script(ExtScript),
    Tmux(ExtTmux)
}

impl ExtExecutables {
    pub fn is_script(&self) -> bool {
        matches!(self, ExtExecutables::Script(_))
    }

    pub fn is_tmux(&self) -> bool {
        matches!(self, ExtExecutables::Tmux(_))
    }

    pub fn get_script_ref(&self) -> &ExtScript {
        match self {
            ExtExecutables::Script(script) => script,
            _ => panic!("Get script on non-script")
        }
    }

    pub fn get_tmux_ref(&self) -> &ExtTmux {
        match self {
            ExtExecutables::Tmux(tmux) => tmux,
            _ => panic!("Get tmux on non-tmux")
        }
    }

    pub fn get_script_mut(&mut self) -> &mut ExtScript {
        match self {
            ExtExecutables::Script(script) => script,
            _ => panic!("Get script on non-script")
        }
    }

    pub fn get_tmux_mut(&mut self) -> &mut ExtTmux {
        match self {
            ExtExecutables::Tmux(tmux) => tmux,
            _ => panic!("Get tmux on non-tmux")
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExtTmux {
    #[serde(default)]
    pub location: Location,
    pub name: Option<String>,
    pub destination: String,
    #[serde(flatten)]
    pub tmux: TmuxBuilder 
}

impl ToInternal<(Script, DataTypes)> for ExtTmux {
    fn to_internal(mut self) -> (Script, DataTypes) {
        let script = Script { name: self.name.clone(), path: self.destination.clone() };

        let data = VolatileData { name: self.name, content: self.tmux.build(), dst: self.destination.into()  };

        let data_type = match self.location {
            Location::Node => VolatileNodeData(data),
            Location::Container => VolatileDockerData(data)
        };

        (script, data_type)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub struct ExtScript {
    pub location: Location,
    pub name: Option<String>,
    pub destination: String
}

impl ToInternal<Script> for ExtScript {
    fn to_internal(self) -> Script {
        Script { name: self.name, path: self.destination }
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
