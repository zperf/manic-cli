use std::{fmt, path::Path};

use clap::{Parser, ValueEnum};
use indicatif::{ProgressState, ProgressStyle};
use manic_cli::{Downloader, ManicError};

#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
enum Method {
    #[value(name = "GET", alias = "get", alias = "Get")]
    GET,

    #[value(name = "PUT", alias = "put", alias = "Put")]
    PUT,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// File url
    url: String,

    /// Method
    #[arg(short, long, value_enum, default_value_t = Method::GET)]
    method: Method,

    /// Output
    #[arg(short, long, default_value_t = String::from("."))]
    output: String,

    /// Filename
    #[arg(short, long)]
    filename: Option<String>,

    /// Thread count
    #[arg(short, long, default_value_t = 4)]
    threads: u8,
}

fn new_style() -> ProgressStyle {
    ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn fmt::Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-")
}

#[tokio::main]
async fn main() -> Result<(), ManicError> {
    let cli = Cli::parse();

    let mut client = Downloader::new(&cli.url, cli.threads).await?;
    let client = client.progress_bar();
    client.bar_style(new_style());

    let filename = cli.filename.unwrap_or(client.filename().to_string());
    let filepath = Path::new(&cli.output).join(filename);

    let chunks = client.download().await?;
    chunks.save_to_file(filepath).await?;
    Ok(())
}
