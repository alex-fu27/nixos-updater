pub struct FlakeConfig {
	pub url: String,
	pub attribute: String,
}

impl FlakeConfig {
	pub fn new(url: &str, attr: &str) -> Self {
		Self {
			url: url.to_string(),
			attribute: attr.to_string()
		}
	}

	pub fn from_url_and_config_name(url: &str, config_name: &str) -> Self {
		Self {
			url: url.to_string(),
			attribute: format!("nixosConfigurations.\"{config_name}\".config.system.build.toplevel"),
		}
	}

	pub fn get_installable(&self) -> String {
		format!("{}#{}", &self.url, &self.attribute)
	}
}


