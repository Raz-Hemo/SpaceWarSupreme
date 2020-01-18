use std::f64::consts::PI as pi;

pub struct MainSequenceStar
{
    radius: f64, // in km
    temperature: f64, // in kelvin
    pos: (f32, f32), // relative to center of galaxy
    name: String,
}

pub enum StarClass
{
    ClassO,
    ClassB,
    ClassA,
    ClassF,
    ClassG,
    ClassK,
    ClassM,
}

impl MainSequenceStar
{
    pub fn get_luminosity(&self) -> f64 // in watts
    {
        // L = r^2            *          T^4               * 7.125e-7
        self.radius.powf(2.0) * self.temperature.powf(4.0) * 0.0000007125_f64
    }

    pub fn get_flux_density(&self) -> f64 // in watt/m^2
    {
        // F = L              /  (4pi*r^2)
        self.get_luminosity() / (4.0f64 * pi * self.radius.powf(2.0))
    }

    pub fn get_class(&self) -> Option<StarClass>
    {
        match self.temperature as i32
        {
            2400..=3699 => Some(StarClass::ClassM),
            3700..=5199 => Some(StarClass::ClassK),
            5200..=5999 => Some(StarClass::ClassG),
            6000..=7499 => Some(StarClass::ClassF),
            7500..=9999 => Some(StarClass::ClassA),
            10000..=29999 => Some(StarClass::ClassB),
            30000..=49999 => Some(StarClass::ClassO),
            _ => None
        }
    }
}