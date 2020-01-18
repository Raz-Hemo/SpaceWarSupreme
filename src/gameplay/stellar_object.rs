

pub struct StellarObject
{
    radius: f64, // in solar radii
    mass: f64, // in solar masses
    pos: (f32, f32), // relative to center of galaxy, normalized to [-1,1]
    name: String,
    info: HashMap<String, LuaDataTypes>,
}

// Some ideas:
// MainSequenceStar(temperature: f64),
// Giant,
// Supergiant,
// Hypergiant,
// WhiteDwarf,
// BrownDwarf,
// BlackHole,
// NeutronStar(MagneticField, RotationRate),