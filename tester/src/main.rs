use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::{atomic::AtomicU64, Arc},
    time::Duration,
};

use clap::Parser;
use cli::Args;
use futures::StreamExt;
use quic_geyser_client::{client::Client, DEFAULT_MAX_STREAM};
use quic_geyser_common::filters::{AccountFilter, Filter};
use quic_geyser_plugin::config::{CompressionParameters, Config, ConfigQuicPlugin, QuicParameters};
use serde_json::json;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair};
use tokio::pin;

pub mod cli;

#[tokio::main]
async fn main() {
    let config = Config {
        libpath: "temp".to_string(),
        quic_plugin: ConfigQuicPlugin {
            address: std::net::SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 10800)),
            quic_parameters: QuicParameters {
                max_number_of_streams_per_client: 1024,
                recieve_window_size: 1_000_000,
                connection_timeout: 600,
            },
            compression_parameters: CompressionParameters {
                compression_type: quic_geyser_common::compression::CompressionType::Lz4Fast(8),
            },
            number_of_retries: 100,
        },
    };
    let config_json = json!(config);
    println!("{}", config_json);

    let args = Args::parse();
    let client = Client::new(args.url, &Keypair::new(), DEFAULT_MAX_STREAM)
        .await
        .unwrap();

    let bytes_transfered = Arc::new(AtomicU64::new(0));
    let slot_notifications = Arc::new(AtomicU64::new(0));
    let account_notification = Arc::new(AtomicU64::new(0));
    let blockmeta_notifications = Arc::new(AtomicU64::new(0));

    let cluster_slot = Arc::new(AtomicU64::new(0));
    let account_slot = Arc::new(AtomicU64::new(0));
    let slot_slot = Arc::new(AtomicU64::new(0));
    let blockmeta_slot = Arc::new(AtomicU64::new(0));

    {
        let cluster_slot = cluster_slot.clone();
        let rpc = RpcClient::new(args.rpc_url);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(100)).await;
                let slot = rpc
                    .get_slot_with_commitment(CommitmentConfig::processed())
                    .await
                    .unwrap();
                cluster_slot.store(slot, std::sync::atomic::Ordering::Relaxed);
            }
        });
    }

    {
        let bytes_transfered: Arc<AtomicU64> = bytes_transfered.clone();
        let slot_notifications = slot_notifications.clone();
        let account_notification = account_notification.clone();
        let blockmeta_notifications = blockmeta_notifications.clone();

        let cluster_slot = cluster_slot.clone();
        let account_slot = account_slot.clone();
        let slot_slot = slot_slot.clone();
        let blockmeta_slot = blockmeta_slot.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                let bytes_transfered =
                    bytes_transfered.swap(0, std::sync::atomic::Ordering::Relaxed);
                log::info!("------------------------------------------");
                log::info!(" Bytes Transfered : {}", bytes_transfered);
                log::info!(
                    " Accounts Notified : {}",
                    account_notification.swap(0, std::sync::atomic::Ordering::Relaxed)
                );
                log::info!(
                    " Slots Notified : {}",
                    slot_notifications.swap(0, std::sync::atomic::Ordering::Relaxed)
                );
                log::info!(
                    " Blockmeta notified : {}",
                    blockmeta_notifications.swap(0, std::sync::atomic::Ordering::Relaxed)
                );

                log::info!(" Cluster Slots: {}, Account Slot: {}, Slot Notification slot: {}, BlockMeta slot: {} ", cluster_slot.load(std::sync::atomic::Ordering::Relaxed), account_slot.load(std::sync::atomic::Ordering::Relaxed), slot_slot.load(std::sync::atomic::Ordering::Relaxed), blockmeta_slot.load(std::sync::atomic::Ordering::Relaxed));
            }
        });
    }

    client
        .subscribe(vec![
            Filter::Account(AccountFilter {
                owner: Some(Pubkey::default()),
                accounts: None,
            }),
            Filter::Slot,
            Filter::BlockMeta,
        ])
        .await
        .unwrap();

    let stream = client.get_stream();
    pin!(stream);

    while let Some(message) = stream.next().await {
        let message_size = bincode::serialize(&message).unwrap().len();
        bytes_transfered.fetch_add(message_size as u64, std::sync::atomic::Ordering::Relaxed);
        match message {
            quic_geyser_common::message::Message::AccountMsg(account) => {
                log::debug!("got account notification : {} ", account.pubkey);
                account_notification.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                account_slot.store(
                    account.slot_identifier.slot,
                    std::sync::atomic::Ordering::Relaxed,
                );
            }
            quic_geyser_common::message::Message::SlotMsg(slot) => {
                log::debug!("got slot notification : {} ", slot.slot);
                slot_notifications.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                slot_slot.store(slot.slot, std::sync::atomic::Ordering::Relaxed);
            }
            quic_geyser_common::message::Message::BlockMetaMsg(block_meta) => {
                log::debug!("got blockmeta notification : {} ", block_meta.slot);
                blockmeta_notifications.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                blockmeta_slot.store(block_meta.slot, std::sync::atomic::Ordering::Relaxed);
            }
            quic_geyser_common::message::Message::Filters(_) => todo!(),
        }
    }
}