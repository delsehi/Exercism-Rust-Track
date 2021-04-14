#[derive(Debug)]
pub struct Duration {
    s: u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self {s}
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

// Define a macro that takes a type and the orbital period and implements the Planet trait
macro_rules! impl_planet {
    // plan = a struct type like Mercury, orbital is the orbital period. 
    ($($plan:ty, $orbital:expr), +) => {
        $(impl Planet for $plan {
            fn years_during(d: &Duration) -> f64 {
                d.s as f64 / 31557600f64 / $orbital
            }
        })*
    };
}
// For each type, implement the type with the period
impl_planet!(Earth, 1f64);
impl_planet!(Mercury, 0.2408467f64);
impl_planet!(Mars, 1.8808158f64);
impl_planet!(Venus, 0.61519726f64);
impl_planet!(Jupiter, 11.862615f64);
impl_planet!(Saturn, 29.447498f64);
impl_planet!(Uranus, 84.016846f64);
impl_planet!(Neptune, 164.79132f64);

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;
