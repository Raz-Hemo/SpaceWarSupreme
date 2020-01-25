#[derive(Clone)]
#[derive(Debug)]
pub struct Star
{
    pub radius: f64, // in km
    pub temperature: f64, // in kelvin
    pub name: String,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct StarSystem
{
    pub pos: (f32, f32), // relative to center of galaxy
    pub name: String,
    pub stars: Vec<Star>,
}
