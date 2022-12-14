use path_clean::PathClean;
use speedtest_tool_fastcom_rs::{
    logger,
    speedtest::{controller, model, recorder, reporter},
};
use std::{env, path};

/// network proxy settings.
#[derive(argh::FromArgs)]
struct Arguments {
    /// proxy server url.
    #[argh(option)]
    proxy_url: Option<String>,
    /// proxy bypass url.
    #[argh(option)]
    proxy_bypass: Option<String>,
    /// username for proxy authentication.
    #[argh(option)]
    proxy_username: Option<String>,
    /// password for proxy authentication.
    #[argh(option)]
    proxy_password: Option<String>,
    /// report upload path
    #[argh(option)]
    upload_path: String,
    /// report save path
    #[argh(option)]
    save_path: String,
    /// bit to byte
    #[argh(switch)]
    convert_byte: bool,
    /// force upload the report.
    #[argh(switch)]
    is_force: bool,
    /// round the datetime.
    #[argh(switch)]
    round_datetime: bool,
}

#[tokio::main]
async fn main() {
    logger::init();

    log::info!("speedtest tool start.");

    println!("Hello, world!");

    let arg: Arguments = argh::from_env();

    let result: model::SpeedTestResultValues = match controller::speedtest(
        arg.proxy_url,
        arg.proxy_bypass,
        arg.proxy_username,
        arg.proxy_password,
    )
    .await
    {
        Ok(value) => value,
        Err(error) => {
            log::error!("Failed speedtest.");
            log::error!("{:?}", error);
            panic!();
        }
    };

    let mut save_path: path::PathBuf = path::PathBuf::from(arg.save_path);
    if save_path.is_relative() && !save_path.starts_with(r"\\") {
        save_path = env::current_dir().unwrap().join(save_path).clean();
    }
    let mut upload_path: path::PathBuf = path::PathBuf::from(arg.upload_path);
    if upload_path.is_relative() && !upload_path.starts_with(r"\\") {
        upload_path = env::current_dir().unwrap().join(upload_path).clean();
    }
    let record_path: path::PathBuf = save_path.join("dest");

    let today: chrono::Date<chrono::Local> = chrono::Local::today();
    let file_path: path::PathBuf =
        record_path.join(format!("{}_fastcom.csv", today.format("%Y-%m-%d")));

    match recorder::record_to_csv(
        file_path.as_path(),
        result,
        arg.convert_byte,
        arg.round_datetime,
    ) {
        Ok(_) => {
            log::info!("Success record to csv.");
        }
        Err(error) => {
            log::error!("Failed record to csv.");
            log::error!("{:?}", error);
            panic!();
        }
    }

    let yesterday: chrono::Date<chrono::Local> = today - chrono::Duration::days(1);

    match reporter::upload_report(
        record_path.as_path(),
        upload_path.as_path(),
        yesterday,
        arg.is_force,
    ) {
        Ok(()) => log::info!("Success upload the report."),
        Err(error) => {
            log::error!("Failed upload the report.");
            log::error!("{:?}", error);
            panic!();
        }
    }

    log::info!("speedtest tool end.");
}
