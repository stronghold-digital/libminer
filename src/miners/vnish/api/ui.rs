use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Cooling {
    pub max_target_temp: f64,
    pub min_fan_pwm: f64,
    pub min_target_temp: f64,
}

#[derive(Deserialize, Debug)]
pub struct Overclock {
    pub default_freq: u32,
    pub default_voltage: u32,
    pub max_freq: u32,
    pub max_voltage: u32,
    pub max_voltage_stock_psu: u32,
    pub min_freq: u32,
    pub min_voltage: u32,
    pub warn_freq: u32,
}

#[derive(Deserialize, Debug)]
pub struct Consts {
    pub cooling: Cooling,
    pub overclock: Overclock,
    pub timezones: Vec<(String, String)>
}

#[derive(Deserialize, Debug)]
pub struct UI {
    pub consts: Consts,
    pub dark_side_pane: bool,
    pub disable_animation: bool,
    pub locale: String,
    pub theme: String,
    pub timezone: String,
}