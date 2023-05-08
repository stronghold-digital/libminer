use lazy_regex::regex;
use serde::Deserialize;

use crate::miner::Profile;

#[derive(Deserialize, Debug, Clone)]
pub struct Preset {
    pub name: String,
    pub pretty: String,
    pub status: String,
    pub modded_psu_required: bool,
}

impl Into<Profile> for Preset {
    fn into(self) -> Profile {
        match self.name.as_str() {
            "disabled" => Profile::Default,
            _ => {
                // 3800 watt ~ 106 Th
                let re = regex!(r"(\d+) Th");
                let caps = re.captures(&self.pretty).unwrap();
                Profile::Preset {
                    power: self.name.parse::<f64>().unwrap(),
                    name: self.name,
                    ths: caps.get(1).unwrap().as_str().parse::<f64>().unwrap(),
                }
            },
        }
    }
}

pub type Presets = Vec<Preset>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preset() {
        let json = r#"{"name":"3800","pretty":"3800 watt ~ 106 Th","status":"untuned","modded_psu_required":false}"#;
        let preset: Preset = serde_json::from_str(json).unwrap();
        let profile: Profile = preset.into();
        match profile {
            Profile::Preset { name, power, ths } => {
                assert_eq!(name, "3800");
                assert_eq!(power, 3800.0);
                assert_eq!(ths, 106.0);
            },
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_presets() {
        let json = r#"[{"name":"disabled","pretty":"Disabled","status":"untuned","modded_psu_required":false},{"name":"2710","pretty":"2710 watt ~ 90 Th","status":"untuned","modded_psu_required":false},{"name":"2850","pretty":"2850 watt ~ 93 Th","status":"untuned","modded_psu_required":false},{"name":"3000","pretty":"3000 watt ~ 96 Th","status":"tuned","modded_psu_required":false},{"name":"3150","pretty":"3150 watt ~ 100 Th","status":"untuned","modded_psu_required":false},{"name":"3320","pretty":"3320 watt ~ 103 Th","status":"untuned","modded_psu_required":false},{"name":"3460","pretty":"3460 watt ~ 106 Th LC","status":"untuned","modded_psu_required":false},{"name":"3640","pretty":"3640 watt ~ 110 Th LC","status":"untuned","modded_psu_required":false}]"#;
        let presets: Presets = serde_json::from_str(json).unwrap();
        assert_eq!(presets.len(), 8);
        let profile: Profile = presets[0].clone().into();
        match profile {
            Profile::Default => {},
            _ => unreachable!(),
        }
        let profile: Profile = presets[1].clone().into();
        match profile {
            Profile::Preset { name, power, ths } => {
                assert_eq!(name, "2710");
                assert_eq!(power, 2710.0);
                assert_eq!(ths, 90.0);
            },
            _ => unreachable!(),
        }
    }
}