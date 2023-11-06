use std::{env, ffi::OsString, process::{Command, Stdio}};

use anyhow::Context;
use cargo_metadata::Metadata;
use semver::{Version, VersionReq};


#[derive(Debug)]
pub struct DepsVersion(Version);

// pub const VER_0_7_T_0_8: &'static str = ">=0.7.0, <0.8.0";

impl DepsVersion {
    pub fn check(&self, version_rng: &str) -> bool {
        let req = VersionReq::parse(version_rng).unwrap();
        req.matches(&self.0)
    }

    pub fn get_version(deps_name: &str) -> Option<DepsVersion> {
        let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
        let mut command = Command::new(cargo);
        command.arg("metadata").arg("--format-version").arg("1");
        let output = command
            .stderr(Stdio::inherit())
            .output()
            .with_context(|| format!("error running '{}'", "cargo metadata"))
            .unwrap();
        let output = String::from_utf8(output.stdout).with_context(|| format!("error parsing {} output", "cargo metadata")).unwrap();

        let metadata: Metadata = serde_json::from_str(&output).context("error parsing cargo metadata output").unwrap();

        let mut rst = None;
        for package in metadata.packages {
            if package.name == deps_name {
                rst = Some(DepsVersion(package.version));

                break;
            }
        }

        rst
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SqlxVer {
    V0_7,
    V0_6,
    V0_5,
    NotSupport(Version),
}

pub(crate) fn get_sqlx_ver() -> Option<SqlxVer> {
    let deps_ver_rst = DepsVersion::get_version("sqlx");

    deps_ver_rst.map(|deps_ver| {
        if deps_ver.0.major == 0 {
            match deps_ver.0.minor  {
                9 => SqlxVer::V0_7,
                8 => SqlxVer::V0_7,
                7 => SqlxVer::V0_7,
                6 => SqlxVer::V0_6,
                5 => SqlxVer::V0_5,
                _ => SqlxVer::NotSupport(deps_ver.0),
            }
        } else {
            SqlxVer::V0_7
        }
    })
}

