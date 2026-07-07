struct Temperature {
    celsius: f64,
}

impl Temperature {
    fn to_fahrenheit(&self) -> f64 {
        (self.celsius * 9.0 / 5.0) + 32.0
    }
    fn to_kelvin(&self) -> f64 {
        self.celsius + 273.15
    }

    fn from_fahrenheit(f: f64) -> Self {
        Temperature {
            celsius: (f - 32.0) * 5.0 / 9.0,
        }
    }
}

fn main() {
    let temperature = Temperature::from_fahrenheit(15.11);
    println!("To Fahrenheit: {}", temperature.to_fahrenheit());
    println!("To Kalvin: {}", temperature.to_kelvin());
}
