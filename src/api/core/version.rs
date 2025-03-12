use crate::{config::Config, error::ApiError};
use axum::{extract::State, Json};
use semver::Version;
use serde::{Deserialize, Serialize};

const MIN_UPDATE_SUPPORT_VERSION_CODE: &str = "1000003";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VersionInfo {
    pub(crate) version_latest: String,
    pub(crate) version_latest_detail: String,
    pub(crate) version_latest_code: String,
    pub(crate) version_latest_note: String,
    pub(crate) min_update_support_version: String,
    pub(crate) new_version_url_prefix: String,
    pub(crate) updater_url_prefix: String,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Channel {
    Stable,
    Beta,
    Dev,
}

impl VersionInfo {
    pub(crate) fn new(domain: &str, channel: Channel, versions: &[&str]) -> Self {
        let common_download_url_prefix = format!("{}/dice/api/core/download", domain);
        let version = Self::select_version(channel, versions);
        Self {
            version_latest: Self::parse_simple_version(channel, &version),
            version_latest_detail: version.to_string(),
            version_latest_code: Self::parse_version_code(channel, &version),
            version_latest_note: String::new(),
            min_update_support_version: MIN_UPDATE_SUPPORT_VERSION_CODE.to_string(),
            new_version_url_prefix: common_download_url_prefix.clone(),
            updater_url_prefix: common_download_url_prefix.clone(),
        }
    }

    fn select_version(_channel: Channel, versions: &[&str]) -> Version {
        let version = versions
            .iter()
            .map(|v| Version::parse(v).unwrap())
            .max()
            .unwrap();
        version
    }

    fn parse_simple_version(channel: Channel, version: &Version) -> String {
        match channel {
            Channel::Stable => {
                format!("{}.{}.{}", version.major, version.minor, version.patch)
            }
            Channel::Beta | Channel::Dev => {
                format!(
                    "{}.{}.{}-{}",
                    version.major, version.minor, version.patch, version.pre
                )
            }
        }
    }

    fn parse_version_code(_channel: Channel, version: &Version) -> String {
        format!("{}{:03}.{:03}", version.major, version.minor, version.patch)
    }
}

pub(crate) async fn version(State(config): State<Config>) -> Result<Json<VersionInfo>, ApiError> {
    let channel = Channel::Stable;
    let server_config = config.server;
    let versions = &["1.4.6", "1.4.7-dev"]; // TODO: parse version from file name
    let version_info = VersionInfo::new(&server_config.domain, channel, versions);
    Ok(Json(version_info))
}
