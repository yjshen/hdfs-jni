use thiserror::Error;

/// Errors which can occur during accessing Hdfs cluster
#[derive(Error, Debug)]
pub enum HdfsErr {
    #[error("Unknown hdfs error")]
    Unknown,
    /// file path
    #[error("File not found `{0}`")]
    FileNotFound(String),
    /// file path           
    #[error("File already exists `{0}`")]
    FileAlreadyExists(String),
    /// namenode address
    #[error("Cannot connect to NameNode `{0}`")]
    CannotConnectToNameNode(String),
    /// URL
    #[error("Invalid URL `{0}`")]
    InvalidUrl(String),
}
