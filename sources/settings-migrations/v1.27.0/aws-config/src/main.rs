use migration_helpers::common_migrations::{AddMetadataMigration, SettingMetadata};
use migration_helpers::{migrate, Result};
use std::process;

///  We added new settings metadata, `aws.config.setting-generator`
fn run() -> Result<()> {
    migrate(AddMetadataMigration(&[SettingMetadata {
        metadata: &["setting-generator"],
        setting: "settings.aws.config",
    }]))
}

// Returning a Result from main makes it print a Debug representation of the error, but with Snafu
// we have nice Display representations of the error, so we wrap "main" (run) and print any error.
// https://github.com/shepmaster/snafu/issues/110
fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
