use std::str::FromStr;

#[derive(Debug)]
pub enum OsSystems {
    Debian,
    Alpine,
    RedHatEnterprise,
    Ubuntu
}

impl FromStr for OsSystems {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        if s.contains("alpine") {
            return Ok(OsSystems::Alpine);
        }
        else if s.contains("debian") {
            return Ok(OsSystems::Debian);
        }
        else if s.contains("ubuntu") {
            return Ok(OsSystems::Ubuntu);
        }
        else if s.contains("rhel") {
            return Ok(OsSystems::RedHatEnterprise);
        }

        Err(())
    } 
}
