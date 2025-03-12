use anyhow::Error;
use log::info;
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::io::Write;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Config {
    pub(crate) server: ServerConfig,
    pub(crate) database: DbConfig,
    pub(crate) core: CoreConfig,
    pub(crate) store: StoreConfig,
    pub(crate) story_log: StoryLogConfig,
}

impl Config {
    pub(crate) fn new(
        server: ServerConfig,
        database: DbConfig,
        core: CoreConfig,
        store: StoreConfig,
        story_log: StoryLogConfig,
    ) -> Self {
        Self {
            server,
            database,
            core,
            store,
            story_log,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct ServerConfig {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) domain: String,
}

impl ServerConfig {
    pub(crate) fn new(host: &str, port: u16, domain: &str) -> Self {
        Self {
            host: host.to_string(),
            port,
            domain: domain.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub(crate) enum DbConfig {
    Sqlite { path: String },
    Postgres { url: String },
}

impl DbConfig {
    pub(crate) fn new_sqlite(path: &str) -> Self {
        Self::Sqlite {
            path: path.to_string(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_postgres(url: &str) -> Self {
        Self::Postgres {
            url: url.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct CoreConfig {
    pub(crate) file_dir: String,
}

impl CoreConfig {
    pub(crate) fn new(file_dir: &str) -> Self {
        Self {
            file_dir: file_dir.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct StoreConfig {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) announcement: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) upload_notice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) upload_form: Option<Vec<UploadFormElem>>,
    pub(crate) extension_dir: String,
}

impl StoreConfig {
    pub(crate) fn new(id: &str, name: &str, announcement: &str, extension_dir: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            announcement: announcement.to_string(),
            upload_notice: None,
            upload_form: None,
            extension_dir: extension_dir.to_string(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn with_upload_notice(mut self, upload_notice: &str) -> Self {
        self.upload_notice = Some(upload_notice.to_string());
        self
    }

    #[allow(dead_code)]
    pub(crate) fn with_upload_form(mut self, upload_form: Vec<UploadFormElem>) -> Self {
        self.upload_form = Some(upload_form);
        self
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct UploadFormOption {
    key: String,
    desc: String,
}

impl UploadFormOption {
    #[allow(dead_code)]
    pub(crate) fn new(key: &str, desc: &str) -> Self {
        Self {
            key: key.to_string(),
            desc: desc.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct UploadFormElem {
    key: String,
    desc: String,
    required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Vec<UploadFormOption>>,
}

impl UploadFormElem {
    #[allow(dead_code)]
    pub(crate) fn new(key: &str, desc: &str, required: bool) -> Self {
        Self {
            key: key.to_string(),
            desc: desc.to_string(),
            required,
            default: None,
            options: None,
        }
    }

    #[allow(dead_code)]
    fn with_default(mut self, default: &str) -> Self {
        self.default = Some(default.to_string());
        self
    }

    #[allow(dead_code)]
    fn with_options(mut self, options: Vec<UploadFormOption>) -> Self {
        self.options = Some(options);
        self
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct StoryLogConfig {
    pub(crate) max_log_mb: usize,
}

impl StoryLogConfig {
    pub(crate) fn new(max_log_mb: usize) -> Self {
        Self { max_log_mb }
    }
}

pub(crate) fn read_config() -> Result<Config, Error> {
    if let Ok(config) = read_to_string("config.toml") {
        info!("The config.toml found, using it");
        let conf: Config = toml::from_str(config.as_str()).expect(
            "Failed to read config.toml. Check the file or delete it to generate a new one",
        );
        Ok(conf)
    } else {
        // info!("config.toml not found. starting interactive setup...");
        info!("The config.toml not found. Generating default config...");
        setup()
    }
}

fn setup() -> Result<Config, Error> {
    let mut file = File::create("config.toml")?;
    let conf = Config::new(
        ServerConfig::new("0.0.0.0", 3210, "http://localhost:3212"),
        DbConfig::new_sqlite("data.db"),
        CoreConfig::new("core-files"),
        StoreConfig::new("seal-store:test", "海豹扩展商店[测试]", "", "extensions"),
        // .with_upload_form(vec![
        //     UploadFormElem::new("name", "名称", true),
        //     UploadFormElem::new("type", "类型", true).with_options(vec![
        //         UploadFormOption::new("plugin", "插件"),
        //         UploadFormOption::new("deck", "牌堆"),
        //     ]),
        //     UploadFormElem::new("version", "版本", true).with_default("1.0.0"),
        //     UploadFormElem::new("author", "作者", true),
        //     UploadFormElem::new("desc", "描述", true),
        // ]),
        StoryLogConfig::new(10),
    ); // TODO interactive setup
    let data = toml::to_string(&conf)?;
    file.write_all(data.as_bytes())?;
    Ok(conf)
}
