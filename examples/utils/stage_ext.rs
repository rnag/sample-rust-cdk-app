use std::fs::read_to_string;

use sample_rust_cdk_app::{Environment, Result, StageConfig};

pub trait StageExt {
    fn stage<'a>(self) -> &'a str;
    fn stage_config<'a>(self) -> Result<StageConfig<'a>>;
}

impl StageExt for Environment {
    /// Return the name of the TOML config in the `./stage_configs` directory.
    fn stage<'a>(self) -> &'a str {
        match self {
            Environment::Prod | Environment::Staging => "prod",
            Environment::Dev | Environment::Sandbox => "dev",
        }
    }

    /// Returns the (deserialized) config from the TOML stage file.
    ///
    /// For more details, please see [my post here].
    ///
    /// [my post here]: https://stackoverflow.com/q/73379769/10237506
    fn stage_config<'a>(self) -> Result<StageConfig<'a>> {
        let file_path = format!("./stage_configs/{}.toml", self.stage());
        debug!("Retrieving TOML config: {file_path}");

        let config: &'a str = Box::leak(read_to_string(file_path)?.into_boxed_str());

        Ok(toml::from_str(config)?)
    }
}
