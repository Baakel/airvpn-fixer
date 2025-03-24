use thiserror::Error;

#[derive(Debug, Error)]
pub enum AirError {
    #[error("Error while setting up connections: {0:?}")]
    IO(#[from] std::io::Error),
    #[error("Zbussing here: {0:?}")]
    Zbus(#[from] zbus::Error),
}
