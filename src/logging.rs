use env_logger::Env;

use crate::constants;

pub fn set_up_logging() {
    env_logger::Builder::from_env(
        Env::default()
            .filter(constants::LOG_ENV_NAME)
            .default_filter_or(constants::LOGGING_DEFAULT_THRESHOLD),
    )
    .init()
}
