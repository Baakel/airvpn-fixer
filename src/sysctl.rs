use crate::models::errors::AirError;
use tracing::info;
use zbus::Connection;

pub async fn renew_bluetit() -> Result<(), AirError> {
    // This didn't work. Think it started the process as something else..?
    // It was probably due to the process not running as root. This could be a viable alternative
    // let mut sysctl = std::process::Command::new("systemctl");
    // let cmd = sysctl.args(["restart", "bluetit"]);
    // info!("Command returned {:#?}", cmd.output().unwrap());
    //

    let connection = Connection::system().await?;

    let proxy = zbus::Proxy::new(
        &connection,
        "org.freedesktop.systemd1",
        "/org/freedesktop/systemd1",
        "org.freedesktop.systemd1.Manager",
    )
    .await?;

    let service_name = "bluetit.service";
    let mode = "replace";

    let body = proxy
        .call_method("RestartUnit", &(service_name, mode))
        .await?
        .to_string();

    info!("sent the restart command. Response: {:#?}", body);
    Ok(())
}
