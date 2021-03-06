#[macro_use]
extern crate prettytable;

use anyhow::anyhow;
use chrono::prelude::DateTime;
use clap::{crate_version, App, AppSettings};
use healthchecks::manage;
use prettytable::{format, Table};
use std::env::var;

#[derive(Debug)]
struct Settings {
    token: String,
    ua: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let ua = match var("HEALTHCHECKS_USERAGENT") {
        Ok(f) => Some(f),
        Err(_) => None,
    };
    let settings = Settings {
        token: var("HEALTHCHECKS_TOKEN").expect("HEALTHCHECKS_TOKEN must be set to run monitor"),
        ua,
    };

    let matches = App::new("hcctl")
        .version(crate_version!())
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(App::new("list").about("Lists the checks in your account with their last ping"))
        .get_matches();

    match matches.subcommand() {
        ("list", _) => list(settings)?,
        (cmd, _) => return Err(anyhow!("unknown subcommand: {}", cmd)),
    }

    Ok(())
}

fn list(settings: Settings) -> anyhow::Result<()> {
    let api = manage::get_config(settings.token, settings.ua)?;
    let checks = api.get_checks()?;

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["Name", "Last Ping"]);

    for check in checks {
        let date = if let Some(date_str) = check.last_ping {
            let date = DateTime::parse_from_rfc3339(&date_str)?;
            date.to_rfc2822().to_string()
        } else {
            "-".to_owned()
        };
        table.add_row(row![check.name, date]);
    }

    table.printstd();

    Ok(())
}
