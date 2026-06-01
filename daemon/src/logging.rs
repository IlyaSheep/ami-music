use std::{fs::create_dir_all, time::SystemTime};

use anyhow::{Context, Result};

pub fn setup_logger() -> Result<()> {
    let data_dir = dirs::data_dir().context("Failed to get DATA DIR.")?;
    let log_path = data_dir.join("ami-music").join("ami-daemon.log");
    if let Some(parent) = log_path.parent() {
        create_dir_all(parent)?;
    }

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("lofty", log::LevelFilter::Error)
        .level_for("zbus", log::LevelFilter::Error)
        .level_for("tracing", log::LevelFilter::Error)
        .level_for("symphonia", log::LevelFilter::Error)
        .level_for("symphonia_core", log::LevelFilter::Error)
        .level_for("symphonia_format_ogg", log::LevelFilter::Error)
        .chain(
            fern::log_file(&log_path)
                .with_context(|| format!("Failed to open log file: {:?}", log_path))?,
        )
        .apply()?;
    Ok(())
}
