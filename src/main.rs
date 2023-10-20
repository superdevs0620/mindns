use env_logger::fmt::Color;
use log::LevelFilter;
use protocol::Result;

use crate::config::Config;
use crate::networking::handler::handle_request;
use crate::networking::udp_serv::UdpServer;
use crate::protocol::byte_packet_buffer::BytePacketBuffer;
use crate::rules::Rule;

mod config;
mod dns;
mod networking;
mod protocol;
mod rules;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger.
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .format(|buf, record| {
            // with colors
            use std::io::Write;

            // Time color.
            let mut time_style = buf.style();
            time_style.set_color(Color::Cyan);

            // Get color for level.
            let level_style = buf.default_level_style(record.level());

            // Get color for target.
            let mut target_style = buf.style();
            target_style.set_color(Color::Magenta);

            writeln!(
                buf,
                "[{}] {} {} - {}",
                time_style.value(chrono::Local::now().format("%Y-%m-%d %H:%M:%S")),
                level_style.value(record.level()),
                target_style.value(record.target()),
                record.args()
            )
        })
        .init();

    // Load configuration file.
    let config = config::load_config_relative("./mindns.toml");
    log::info!("Loaded configuration file.");

    // Load rules.
    let rules = rules::parse_rules_config(&config.rules);
    log::info!("Loaded {} rules.", rules.len());

    // Start DNS server.
    let raw_addr = format!("{}:{}", config.server.bind, config.server.port);
    log::info!("Starting DNS server at udp://{}", raw_addr);

    UdpServer::new(
        raw_addr,
        |peer, mut reader, (config, rules): (Config, Vec<Rule>)| async move {
            let mut buffer = BytePacketBuffer::new();
            while let Some(Ok(data)) = reader.recv().await {
                buffer.pos = 0;
                buffer.buf[..data.len()].copy_from_slice(&data);

                handle_request(&config, &rules, &peer, &mut buffer).await?;
            }

            Ok(())
        },
    )?
    .set_peer_timeout_sec(20)
    .start((config, rules))
    .await?;

    Ok(())
}
