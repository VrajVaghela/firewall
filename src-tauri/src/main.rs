#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if let Err(error) = firewall_lib::run() {
        tracing::error!(%error, "Firewall exited unexpectedly");
    }
}
