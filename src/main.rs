// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::iced::Size;
use cosmic::{
    app::{Application, Settings},
    cosmic_config::{self, CosmicConfigEntry},
    iced::Limits,
};
use done_core::service::Services;

use app::{App, Flags};
use config::{Config, CONFIG_VERSION};

mod app;
mod config;
mod content;
mod details;
mod key_bind;
mod localize;
mod menu;
mod todo;

#[rustfmt::skip]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    localize::localize();

    Services::init(App::APP_ID);

    let (config_handler, config) = match cosmic_config::Config::new(App::APP_ID, CONFIG_VERSION) {
        Ok(config_handler) => {
            let config = Config::get_entry(&config_handler).unwrap_or_else(|(errs, config)| {
                log::info!("errors loading config: {:?}", errs);
                config
            });
            (Some(config_handler), config)
        }
        Err(err) => {
            log::error!("failed to create config handler: {}", err);
            (None, Config::default())
        }
    };

    let mut settings = Settings::default();
    settings = settings.theme(config.app_theme.theme());
    settings = settings.size_limits(Limits::NONE.min_width(400.0).min_height(180.0));
    settings = settings.size(Size::new(700.0, 800.0));
    settings = settings.debug(false);

    let flags = Flags {
        config_handler,
        config,
    };

    cosmic::app::run::<App>(settings, flags)?;

    Ok(())
}
