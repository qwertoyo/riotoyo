use chrono::{NaiveDateTime, DateTime, Utc};
use dht_sensor::dht11::{Reading};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Measure{
    timestamp: i64,
    temperature: f32,  // °C
    humidity: f32, // %
}

impl Measure{
    pub fn new (timestamp: i64, temperature: f32, humidity: f32) -> Self{ Self{ timestamp, temperature, humidity} }
    pub fn from_dht11(dht_measure: Reading) -> Measure{
        Measure { 
            timestamp: chrono::Utc::now().timestamp(),
            temperature: dht_measure.temperature as f32 / 10.0, // The measured temperature in tenths of degrees Celsius.
            humidity: dht_measure.relative_humidity as f32 / 10.0 // The measured humidity in tenths of a percent.
        }
    }

    pub fn get_temp_celsius(self: &Measure) -> f32{ self.temperature }
    pub fn get_humidity_percentage(self: &Measure) -> f32 { self.humidity }
    pub fn get_datetime(self: &Measure) -> DateTime<Utc> { 
        let naive_dt = NaiveDateTime::from_timestamp_opt(self.timestamp, 0).unwrap();
        DateTime::from_utc(naive_dt, Utc)
    }
}

