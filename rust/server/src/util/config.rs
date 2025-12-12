use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
	pub(crate) server_config: ServerConfig,
	pub(crate) postgresql_config: Option<PostgreSQLConfig>,
}

#[derive(Deserialize)]
pub(crate) struct ServerConfig {
	pub(crate) host: Option<String>,  // Optional in TOML, can be overridden by env
	pub(crate) port: Option<u16>,     // Optional in TOML, can be overridden by env
	pub(crate) jwt_public_key: Option<String>, // Optional in TOML, can be overridden by env
}

impl ServerConfig {
	pub(crate) fn get_host(&self) -> String {
		std::env::var("VSS_SERVER_HOST")
			.ok()
			.or_else(|| self.host.clone())
			.expect("Server host must be provided in config or env var VSS_SERVER_HOST must be set.")
	}

	pub(crate) fn get_port(&self) -> u16 {
		std::env::var("VSS_SERVER_PORT")
			.ok()
			.and_then(|p| p.parse().ok())
			.or(self.port)
			.expect("Server port must be provided in config or env var VSS_SERVER_PORT must be set.")
	}

	pub(crate) fn get_jwt_public_key(&self) -> String {
		// First check for direct key in environment variable
		if let Ok(key) = std::env::var("VSS_JWT_PUBLIC_KEY") {
			return key;
		}
		// Then check for key file path in environment variable
		if let Ok(key_path) = std::env::var("VSS_JWT_PUBLIC_KEY_FILE") {
			match std::fs::read_to_string(&key_path) {
				Ok(key) => return key,
				Err(e) => {
					panic!("Failed to read JWT public key from {}: {}", key_path, e);
				}
			}
		}
		// Finally check config file
		if let Some(key) = &self.jwt_public_key {
			return key.clone();
		}
		panic!("JWT public key must be provided in config.toml or one of env vars VSS_JWT_PUBLIC_KEY or VSS_JWT_PUBLIC_KEY_FILE must be set.")
	}
}

#[derive(Deserialize)]
pub(crate) struct PostgreSQLConfig {
	pub(crate) username: Option<String>,	// Optional in TOML, can be overridden via env var
	pub(crate) password: Option<String>,	// Optional in TOML, can be overridden via env var
	pub(crate) host: Option<String>,		// Optional in TOML, can be overridden via env var
	pub(crate) port: Option<u16>,			// Optional in TOML, can be overridden via env var
	pub(crate) database: Option<String>,	// Optional in TOML, can be overridden via env var
	pub(crate) tls: Option<TlsConfig>,
}

#[derive(Deserialize)]
pub(crate) struct TlsConfig {
	pub(crate) ca_file: Option<String>,
}

impl PostgreSQLConfig {
	pub(crate) fn to_postgresql_endpoint(&self) -> String {
		let username_env = std::env::var("VSS_POSTGRESQL_USERNAME");
		let username = username_env.as_ref()
			.ok()
			.or_else(|| self.username.as_ref())
			.expect("PostgreSQL database username must be provided in config or env var VSS_POSTGRESQL_USERNAME must be set.");
		let password_env = std::env::var("VSS_POSTGRESQL_PASSWORD");
		let password = password_env.as_ref()
			.ok()
			.or_else(|| self.password.as_ref())
			.expect("PostgreSQL database password must be provided in config or env var VSS_POSTGRESQL_PASSWORD must be set.");
		let host = std::env::var("VSS_POSTGRESQL_HOST")
			.ok()
			.or_else(|| self.host.clone())
			.expect("PostgreSQL database host must be provided in config or env var VSS_POSTGRESQL_HOST must be set.");
		let port = std::env::var("VSS_POSTGRESQL_PORT")
			.ok()
			.and_then(|p| p.parse().ok())
			.or(self.port)
			.expect("PostgreSQL database port must be provided in config or env var VSS_POSTGRESQL_PORT must be set.");

		format!("postgresql://{}:{}@{}:{}", username, password, host, port)
	}
}

pub(crate) fn load_config(config_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
	let config_str = std::fs::read_to_string(config_path)?;
	let config: Config = toml::from_str(&config_str)?;
	Ok(config)
}
