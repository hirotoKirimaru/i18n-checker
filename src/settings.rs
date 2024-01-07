use config::{Config, ConfigError, Environment, File};
// use serde_derive::Deserialize;
use std::env;


// lazy_static! {
//     static ref SETTINGS: RWLock<Config> = RwLock::new(Config::default());
// }

// #[derive(Debug, Deserialize)]
//
// lazy_static! {
//     static ref SETTINGS: Settings;
// }
//
// #[allow(unused)]
// struct Base {
//     base_dir: String,
// }
//
// // #[derive(Debug, Deserialize)]
// #[allow(unused)]
// pub struct Settings {
//     debug: bool,
//     base: Base,
// }
//
// // impl Settings {
// //     pub fn new() -> Result<Self, ConfigError> {
// //         // let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
// //         //
// //         // let s = Config::builder()
// //         //     // Start off by merging in the "default" configuration file
// //         //     .add_source(File::with_name("examples/hierarchical-env/config/default"))
// //         //     // Add in the current environment file
// //         //     // Default to 'development' env
// //         //     // Note that this file is _optional_
// //         //     .add_source(
// //         //         File::with_name(&format!("examples/hierarchical-env/config/{}", run_mode))
// //         //             .required(false),
// //         //     )
// //         //     // Add in a local configuration file
// //         //     // This file shouldn't be checked in to git
// //         //     .add_source(File::with_name("examples/hierarchical-env/config/local").required(false))
// //         //     // Add in settings from the environment (with a prefix of APP)
// //         //     // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
// //         //     .add_source(Environment::with_prefix("app"))
// //         //     // You may also programmatically change settings
// //         //     .set_override("database.url", "postgres://")?
// //         //     .build()?;
// //         let s = Config::builder()
// //             .add_source(File::with_name("config/default"))
// //             .add_source(File::with_name("config/test"))
// //             .build()
// //             .unwrap();
// //
// //         // Now that we're done, let's access our configuration
// //         // println!("debug: {:?}", s.get_bool("debug"));
// //         // println!("database: {:?}", s.get::<String>("database.url"));
// //
// //         // You can deserialize (and thus freeze) the entire configuration as
// //         s.try_deserialize()
// //     }
// // }