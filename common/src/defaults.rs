pub const DEFAULT_MAX_STREAMS: u64 = 512 * 1024;
pub const DEFAULT_MAX_MESSAGES_IN_QUEUE: u64 = 1024 * 1024;
pub const MAX_ALLOWED_PARTIAL_RESPONSES: u64 = DEFAULT_MAX_STREAMS * 3 / 4;
pub const DEFAULT_MAX_RECIEVE_WINDOW_SIZE: u64 = 24 * 1024 * 1024; // 24 MBs
pub const DEFAULT_CONNECTION_TIMEOUT: u64 = 10;
pub const DEFAULT_MAX_NB_CONNECTIONS: u64 = 10;
pub const DEFAULT_MAX_ACK_DELAY: u64 = 25;
pub const DEFAULT_ACK_EXPONENT: u64 = 3;
pub const ALPN_GEYSER_PROTOCOL_ID: &[u8] = b"geyser";
pub const MAX_DATAGRAM_SIZE: usize = 1350; // MAX: 65527
pub const DEFAULT_ENABLE_PACING: bool = true;
pub const DEFAULT_USE_CC_BBR: bool = false;
pub const DEFAULT_INCREMENTAL_PRIORITY: bool = true;
