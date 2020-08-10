use std::collections::HashMap;

use config::{Config, ConfigError, File, Value};
use serde::Deserialize;

use crate::settings::HydroSettings;
use crate::utils::config_locations;

#[derive(Debug, Clone)]
pub struct Hydroconf {
    config: Config,
    hydro: HydroSettings,
}

impl Default for Hydroconf {
    fn default() -> Self {
        Self::new(HydroSettings::default())
    }
}

impl Hydroconf {
    fn new(hydro: HydroSettings) -> Self {
        Self {
            config: Config::default(),
            hydro,
        }
    }

    pub fn hydrate<'de, T: Deserialize<'de>>(
        mut self,
    ) -> Result<T, ConfigError> {
        self.initialize()?;
        self.try_into()
    }

    pub fn initialize(&mut self) -> Result<&mut Self, ConfigError> {
        let root_path = self
            .hydro
            .root_path
            .clone()
            .or_else(|| std::env::current_exe().ok());
        if let Some(p) = root_path {
            let (settings, secrets) = config_locations(p);
            if let Some(settings_path) = settings {
                self.config.merge(File::from(settings_path))?;
            }
            if let Some(secrets_path) = secrets {
                self.config.merge(File::from(secrets_path))?;
            }
        }

        Ok(self)
    }

    pub fn try_into<'de, T: Deserialize<'de>>(self) -> Result<T, ConfigError> {
        self.config.try_into()
    }

    pub fn refresh(&mut self) -> Result<&mut Self, ConfigError> {
        self.config.refresh()?;
        Ok(self)
    }

    pub fn set_default<T>(
        &mut self,
        key: &str,
        value: T,
    ) -> Result<&mut Self, ConfigError>
    where
        T: Into<Value>,
    {
        self.config.set_default(key, value)?;
        Ok(self)
    }

    pub fn set<T>(
        &mut self,
        key: &str,
        value: T,
    ) -> Result<&mut Self, ConfigError>
    where
        T: Into<Value>,
    {
        self.config.set(key, value)?;
        Ok(self)
    }

    pub fn get<'de, T>(&self, key: &'de str) -> Result<T, ConfigError>
    where
        T: Deserialize<'de>,
    {
        self.config.get(key)
    }

    pub fn get_str(&self, key: &str) -> Result<String, ConfigError> {
        self.get(key).and_then(Value::into_str)
    }

    pub fn get_int(&self, key: &str) -> Result<i64, ConfigError> {
        self.get(key).and_then(Value::into_int)
    }

    pub fn get_float(&self, key: &str) -> Result<f64, ConfigError> {
        self.get(key).and_then(Value::into_float)
    }

    pub fn get_bool(&self, key: &str) -> Result<bool, ConfigError> {
        self.get(key).and_then(Value::into_bool)
    }

    pub fn get_table(
        &self,
        key: &str,
    ) -> Result<HashMap<String, Value>, ConfigError> {
        self.get(key).and_then(Value::into_table)
    }

    pub fn get_array(&self, key: &str) -> Result<Vec<Value>, ConfigError> {
        self.get(key).and_then(Value::into_array)
    }
}
