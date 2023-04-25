mod buffer;
mod measure;
use std::{collections::HashMap, thread::sleep, time::Duration};
use config::{Config, ConfigError};



pub fn main() {
    let configuration = Config::builder()
    // Add in `./Settings.toml`
    .add_source(config::File::with_name("src/appsettings.toml"))
    // Add in settings from the environment (with a prefix of APP)
    // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
    .add_source(config::Environment::with_prefix("RIOTOYO"))
    .build()
    .unwrap()
    .try_deserialize::<HashMap<String, String>>()
    .unwrap();

println!(
    "configuration: {:?}",
    configuration
);

match configuration["RIOTOYO_DEVENV"].as_str() {
    "1" => run_dev(),
    "0" => run(),
    _ => panic!("set RIOTOYO_DEVENV to 0 or 1")
}    
let buffer = buffer::Buffer::new();
}


fn run(){

}

fn run_dev(){
    let mut runs_left = 100;
    while runs_left > 0 {
        let measure = measure::Measure::new(
            chrono::Utc::now().timestamp(),
            42.9,
            62.4);

        runs_left -= 1;
        println!("Measure: {:?} - todo {:?}", measure, runs_left);
        sleep(Duration::from_millis(666))
    }
}