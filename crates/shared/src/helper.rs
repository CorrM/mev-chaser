//
// -- Logging Macros --
//
#[macro_export]
macro_rules! log_info_cyan {
    ($($arg:tt)*) => {
        log::info!("{}", format_args!($($arg)*).to_string().cyan());
    };
}

#[macro_export]
macro_rules! startup_info_log {
    ($($arg:tt)*) => {
        log::info!("{}", format_args!($($arg)*).to_string().on_black().yellow().bold());
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        log::error!("{}", format_args!($($arg)*).to_string().red());
    };
}

#[macro_export]
macro_rules! log_new_block_info {
    ($new_block:expr) => {
        log::info!(
            "{}",
            format!(
                "\nFound New Block\nLatest Block: (number:{:?}, timestamp:{:?}, basefee:{:?})",
                $new_block.number, $new_block.timestamp, $new_block.base_fee_per_gas,
            )
            .bright_purple()
            .on_black()
        );
    };
}
