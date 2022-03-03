use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpecError {
    #[error("Download spec file error")]
    DownloadError(#[from] reqwest::Error),
    #[error("Parsing toml file error")]
    ParseError(#[from] toml::de::Error),
    #[error("I/O error")]
    IoError(#[from] std::io::Error),

}
