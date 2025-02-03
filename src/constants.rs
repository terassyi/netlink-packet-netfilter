// SPDX-License-Identifier: MIT

pub use netlink_packet_core::constants::*;

pub const AF_UNSPEC: u8 = libc::AF_UNSPEC as u8;
pub const AF_UNIX: u8 = libc::AF_UNIX as u8;
pub const AF_LOCAL: u8 = libc::AF_LOCAL as u8;
pub const AF_INET: u8 = libc::AF_INET as u8;
pub const AF_AX25: u8 = libc::AF_AX25 as u8;
pub const AF_IPX: u8 = libc::AF_IPX as u8;
pub const AF_APPLETALK: u8 = libc::AF_APPLETALK as u8;
pub const AF_NETROM: u8 = libc::AF_NETROM as u8;
pub const AF_BRIDGE: u8 = libc::AF_BRIDGE as u8;
pub const AF_ATMPVC: u8 = libc::AF_ATMPVC as u8;
pub const AF_X25: u8 = libc::AF_X25 as u8;
pub const AF_INET6: u8 = libc::AF_INET6 as u8;
pub const AF_ROSE: u8 = libc::AF_ROSE as u8;
pub const AF_DECNET: u8 = libc::AF_DECnet as u8;
pub const AF_NETBEUI: u8 = libc::AF_NETBEUI as u8;
pub const AF_SECURITY: u8 = libc::AF_SECURITY as u8;
pub const AF_KEY: u8 = libc::AF_KEY as u8;
pub const AF_NETLINK: u8 = libc::AF_NETLINK as u8;
pub const AF_ROUTE: u8 = libc::AF_ROUTE as u8;
pub const AF_PACKET: u8 = libc::AF_PACKET as u8;
pub const AF_ASH: u8 = libc::AF_ASH as u8;
pub const AF_ECONET: u8 = libc::AF_ECONET as u8;
pub const AF_ATMSVC: u8 = libc::AF_ATMSVC as u8;
pub const AF_RDS: u8 = libc::AF_RDS as u8;
pub const AF_SNA: u8 = libc::AF_SNA as u8;
pub const AF_IRDA: u8 = libc::AF_IRDA as u8;
pub const AF_PPPOX: u8 = libc::AF_PPPOX as u8;
pub const AF_WANPIPE: u8 = libc::AF_WANPIPE as u8;
pub const AF_LLC: u8 = libc::AF_LLC as u8;
pub const AF_CAN: u8 = libc::AF_CAN as u8;
pub const AF_TIPC: u8 = libc::AF_TIPC as u8;
pub const AF_BLUETOOTH: u8 = libc::AF_BLUETOOTH as u8;
pub const AF_IUCV: u8 = libc::AF_IUCV as u8;
pub const AF_RXRPC: u8 = libc::AF_RXRPC as u8;
pub const AF_ISDN: u8 = libc::AF_ISDN as u8;
pub const AF_PHONET: u8 = libc::AF_PHONET as u8;
pub const AF_IEEE802154: u8 = libc::AF_IEEE802154 as u8;
pub const AF_CAIF: u8 = libc::AF_CAIF as u8;
pub const AF_ALG: u8 = libc::AF_ALG as u8;

pub const NFNETLINK_V0: u8 = libc::NFNETLINK_V0 as u8;

pub const NFNL_SUBSYS_NONE: u8 = libc::NFNL_SUBSYS_NONE as u8;
pub const NFNL_SUBSYS_CTNETLINK: u8 = libc::NFNL_SUBSYS_CTNETLINK as u8;
pub const NFNL_SUBSYS_CTNETLINK_EXP: u8 = libc::NFNL_SUBSYS_CTNETLINK_EXP as u8;
pub const NFNL_SUBSYS_QUEUE: u8 = libc::NFNL_SUBSYS_QUEUE as u8;
pub const NFNL_SUBSYS_ULOG: u8 = libc::NFNL_SUBSYS_ULOG as u8;
pub const NFNL_SUBSYS_OSF: u8 = libc::NFNL_SUBSYS_OSF as u8;
pub const NFNL_SUBSYS_IPSET: u8 = libc::NFNL_SUBSYS_IPSET as u8;
pub const NFNL_SUBSYS_ACCT: u8 = libc::NFNL_SUBSYS_ACCT as u8;
pub const NFNL_SUBSYS_CTNETLINK_TIMEOUT: u8 =
    libc::NFNL_SUBSYS_CTNETLINK_TIMEOUT as u8;
pub const NFNL_SUBSYS_CTHELPER: u8 = libc::NFNL_SUBSYS_CTHELPER as u8;
pub const NFNL_SUBSYS_NFTABLES: u8 = libc::NFNL_SUBSYS_NFTABLES as u8;
pub const NFNL_SUBSYS_NFT_COMPAT: u8 = libc::NFNL_SUBSYS_NFT_COMPAT as u8;

pub const NFULA_CFG_CMD: u16 = libc::NFULA_CFG_CMD as u16;
pub const NFULA_CFG_MODE: u16 = libc::NFULA_CFG_MODE as u16;
pub const NFULA_CFG_NLBUFSIZ: u16 = libc::NFULA_CFG_NLBUFSIZ as u16;
pub const NFULA_CFG_TIMEOUT: u16 = libc::NFULA_CFG_TIMEOUT as u16;
pub const NFULA_CFG_QTHRESH: u16 = libc::NFULA_CFG_QTHRESH as u16;
pub const NFULA_CFG_FLAGS: u16 = libc::NFULA_CFG_FLAGS as u16;
pub const NLBUFSIZ_MAX: u32 = 131072;

pub const NFULA_PACKET_HDR: u16 = libc::NFULA_PACKET_HDR as u16;
pub const NFULA_MARK: u16 = libc::NFULA_MARK as u16;
pub const NFULA_TIMESTAMP: u16 = libc::NFULA_TIMESTAMP as u16;
pub const NFULA_IFINDEX_INDEV: u16 = libc::NFULA_IFINDEX_INDEV as u16;
pub const NFULA_IFINDEX_OUTDEV: u16 = libc::NFULA_IFINDEX_OUTDEV as u16;
pub const NFULA_IFINDEX_PHYSINDEV: u16 = libc::NFULA_IFINDEX_PHYSINDEV as u16;
pub const NFULA_IFINDEX_PHYSOUTDEV: u16 = libc::NFULA_IFINDEX_PHYSOUTDEV as u16;
pub const NFULA_HWADDR: u16 = libc::NFULA_HWADDR as u16;
pub const NFULA_PAYLOAD: u16 = libc::NFULA_PAYLOAD as u16;
pub const NFULA_PREFIX: u16 = libc::NFULA_PREFIX as u16;
pub const NFULA_UID: u16 = libc::NFULA_UID as u16;
pub const NFULA_SEQ: u16 = libc::NFULA_SEQ as u16;
pub const NFULA_SEQ_GLOBAL: u16 = libc::NFULA_SEQ_GLOBAL as u16;
pub const NFULA_GID: u16 = libc::NFULA_GID as u16;
pub const NFULA_HWTYPE: u16 = libc::NFULA_HWTYPE as u16;
pub const NFULA_HWHEADER: u16 = libc::NFULA_HWHEADER as u16;
pub const NFULA_HWLEN: u16 = libc::NFULA_HWLEN as u16;
pub const NFULA_CT: u16 = libc::NFULA_CT as u16;
pub const NFULA_CT_INFO: u16 = libc::NFULA_CT_INFO as u16;

pub const NFULNL_MSG_CONFIG: u8 = libc::NFULNL_MSG_CONFIG as u8;
pub const NFULNL_MSG_PACKET: u8 = libc::NFULNL_MSG_PACKET as u8;

// netflter/nfnetlink_conntrack.h
// There is no definitions in rust-lang/libc
pub const IPCTNL_MSG_CT_NEW: u8 = 0;
pub const IPCTNL_MSG_CT_GET: u8 = 1;
pub const IPCTNL_MSG_CT_DELETE: u8 = 2;
pub const IPCTNL_MSG_CT_GET_CTRZERO: u8 = 3;
pub const IPCTNL_MSG_CT_GET_STATS_CPU: u8 = 4;
pub const IPCTNL_MSG_CT_GET_STATS: u8 = 5;
pub const IPCTNL_MSG_CT_GET_DYING: u8 = 6;
pub const IPCTNL_MSG_CT_GET_UNCONFIRMED: u8 = 7;

pub const CTA_UNSPEC: u16 = 0;
pub const CTA_TUPLE_ORIG: u16 = 1;
pub const CTA_TUPLE_REPLY: u16 = 2;
pub const CTA_STATUS: u16 = 3;
pub const CTA_PROTOINFO: u16 = 4;
pub const CTA_HELP: u16 = 5;
pub const CTA_NAT_SRC: u16 = 6;
pub const CTA_NAT: u16 = CTA_NAT_SRC; /* backwards compatibility */
pub const CTA_TIMEOUT: u16 = 7;
pub const CTA_MARK: u16 = 8;
pub const CTA_COUNTERS_ORIG: u16 = 9;
pub const CTA_COUNTERS_REPLY: u16 = 10;
pub const CTA_USE: u16 = 11;
pub const CTA_ID: u16 = 12;
pub const CTA_NAT_DST: u16 = 13;
pub const CTA_TUPLE_MASTER: u16 = 14;
pub const CTA_SEQ_ADJ_ORIG: u16 = 15;
pub const CTA_NAT_SEQ_ADJ_ORIG: u16 = CTA_SEQ_ADJ_ORIG;
pub const CTA_SEQ_ADJ_REPLY: u16 = 16;
pub const CTA_NAT_SEQ_ADJ_REPLY: u16 = CTA_SEQ_ADJ_REPLY;
pub const CTA_SECMARK: u16 = 17; /* obsolete */
pub const CTA_ZONE: u16 = 18;
pub const CTA_SECCTX: u16 = 19;
pub const CTA_TIMESTAMP: u16 = 20;
pub const CTA_MARK_MASK: u16 = 21;
pub const CTA_LABELS: u16 = 22;
pub const CTA_LABELS_MASK: u16 = 23;
pub const CTA_SYNPROXY: u16 = 24;
pub const CTA_FILTER: u16 = 25;
pub const CTA_STATUS_MASK: u16 = 26;

pub const CTA_TUPLE_UNSPEC: u16 = 0;
pub const CTA_TUPLE_IP: u16 = 1;
pub const CTA_TUPLE_PROTO: u16 = 2;
pub const CTA_TUPLE_ZONE: u16 = 3;

pub const CTA_IP_UNSPEC: u16 = 0;
pub const CTA_IP_V4_SRC: u16 = 1;
pub const CTA_IP_V4_DST: u16 = 2;
pub const CTA_IP_V6_SRC: u16 = 3;
pub const CTA_IP_V6_DST: u16 = 4;

pub const CTA_PROTO_UNSPEC: u16 = 0;
pub const CTA_PROTO_NUM: u16 = 1;
pub const CTA_PROTO_SRC_PORT: u16 = 2;
pub const CTA_PROTO_DST_PORT: u16 = 3;
pub const CTA_PROTO_ICMP_ID: u16 = 4;
pub const CTA_PROTO_ICMP_TYPE: u16 = 5;
pub const CTA_PROTO_ICMP_CODE: u16 = 6;
pub const CTA_PROTO_ICMPV6_ID: u16 = 7;
pub const CTA_PROTO_ICMPV6_TYPE: u16 = 8;
pub const CTA_PROTO_ICMPV6_CODE: u16 = 9;

pub const CTA_PROTOINFO_UNSPEC: u16 = 0;
pub const CTA_PROTOINFO_TCP: u16 = 1;
pub const CTA_PROTOINFO_DCCP: u16 = 2;
pub const CTA_PROTOINFO_SCTP: u16 = 3;

pub const CTA_PROTOINFO_TCP_UNSPEC: u16 = 0;
pub const CTA_PROTOINFO_TCP_STATE: u16 = 1;
pub const CTA_PROTOINFO_TCP_WSCALE_ORIGINAL: u16 = 2;
pub const CTA_PROTOINFO_TCP_WSCALE_REPLY: u16 = 3;
pub const CTA_PROTOINFO_TCP_FLAGS_ORIGINAL: u16 = 4;
pub const CTA_PROTOINFO_TCP_FLAGS_REPLY: u16 = 5;

pub const CTA_PROTOINFO_DCCP_UNSPEC: u16 = 0;
pub const CTA_PROTOINFO_DCCP_STATE: u16 = 1;
pub const CTA_PROTOINFO_DCCP_ROLE: u16 = 2;
pub const CTA_PROTOINFO_DCCP_HANDSHAKE_SEQ: u16 = 3;
pub const CTA_PROTOINFO_DCCP_PAD: u16 = 4;

pub const CTA_PROTOINFO_SCTP_UNSPEC: u16 = 0;
pub const CTA_PROTOINFO_SCTP_STATE: u16 = 1;
pub const CTA_PROTOINFO_SCTP_VTAG_ORIGINAL: u16 = 2;
pub const CTA_PROTOINFO_SCTP_VTAG_REPLY: u16 = 3;

pub const CTA_COUNTERS_UNSPEC: u8 = 0;
pub const CTA_COUNTERS_PACKETS: u8 = 1; /* 64bit counters */
pub const CTA_COUNTERS_BYTES: u8 = 2; /* 64bit counters */
pub const CTA_COUNTERS32_PACKETS: u8 = 3; /* old 32bit counters, unused */
pub const CTA_COUNTERS32_BYTES: u8 = 4; /* old 32bit counters, unused */
pub const CTA_COUNTERS_PAD: u8 = 5;

pub const CTA_TIMESTAMP_UNSPEC: u8 = 0;
pub const CTA_TIMESTAMP_START: u8 = 1;
pub const CTA_TIMESTAMP_STOP: u8 = 2;
pub const CTA_TIMESTAMP_PAD: u8 = 3;

pub const CTA_NAT_UNSPEC: u8 = 0;
pub const CTA_NAT_V4_MINIP: u8 = 1;
pub const CTA_NAT_MINIP: u8 = CTA_NAT_V4_MINIP;
pub const CTA_NAT_V4_MAXIP: u8 = 2;
pub const CTA_NAT_MAXIP: u8 = CTA_NAT_V4_MAXIP;
pub const CTA_NAT_PROTO: u8 = 3;
pub const CTA_NAT_V6_MINIP: u8 = 4;
pub const CTA_NAT_V6_MAXIP: u8 = 5;

pub const CTA_PROTONAT_UNSPEC: u8 = 0;
pub const CTA_PROTONAT_PORT_MIN: u8 = 1;
pub const CTA_PROTONAT_PORT_MAX: u8 = 2;

pub const CTA_SEQADJ_UNSPEC: u8 = 0;
pub const CTA_SEQADJ_CORRECTION_POS: u8 = 1;
pub const CTA_SEQADJ_OFFSET_BEFORE: u8 = 2;
pub const CTA_SEQADJ_OFFSET_AFTER: u8 = 3;

pub const CTA_NAT_SEQ_UNSPEC: u8 = 0;
pub const CTA_NAT_SEQ_CORRECTION_POS: u8 = 1;
pub const CTA_NAT_SEQ_OFFSET_BEFORE: u8 = 2;
pub const CTA_NAT_SEQ_OFFSET_AFTER: u8 = 3;

pub const CTA_SYNPROXY_UNSPEC: u8 = 0;
pub const CTA_SYNPROXY_ISN: u8 = 1;
pub const CTA_SYNPROXY_ITS: u8 = 2;
pub const CTA_SYNPROXY_TSOFF: u8 = 3;

pub const CTA_EXPECT_UNSPEC: u8 = 0;
pub const CTA_EXPECT_MASTER: u8 = 1;
pub const CTA_EXPECT_TUPLE: u8 = 2;
pub const CTA_EXPECT_MASK: u8 = 3;
pub const CTA_EXPECT_TIMEOUT: u8 = 4;
pub const CTA_EXPECT_ID: u8 = 5;
pub const CTA_EXPECT_HELP_NAME: u8 = 6;
pub const CTA_EXPECT_ZONE: u8 = 7;
pub const CTA_EXPECT_FLAGS: u8 = 8;
pub const CTA_EXPECT_CLASS: u8 = 9;
pub const CTA_EXPECT_NAT: u8 = 10;
pub const CTA_EXPECT_FN: u8 = 11;

pub const CTA_EXPECT_NAT_UNSPEC: u8 = 0;
pub const CTA_EXPECT_NAT_DIR: u8 = 1;
pub const CTA_EXPECT_NAT_TUPLE: u8 = 2;

pub const CTA_SECCTX_UNSPEC: u8 = 0;
pub const CTA_SECCTX_NAME: u8 = 1;

pub const CTA_STATS_UNSPEC: u16 = 0;
pub const CTA_STATS_SEARCHED: u16 = 1; /* no longer used */
pub const CTA_STATS_FOUND: u16 = 2;
pub const CTA_STATS_NEW: u16 = 3; /* no longer used */
pub const CTA_STATS_INVALID: u16 = 4;
pub const CTA_STATS_IGNORE: u16 = 5; /* no longer used */
pub const CTA_STATS_DELETE: u16 = 6; /* no longer used */
pub const CTA_STATS_DELETE_LIST: u16 = 7; /* no longer used */
pub const CTA_STATS_INSERT: u16 = 8;
pub const CTA_STATS_INSERT_FAILED: u16 = 9;
pub const CTA_STATS_DROP: u16 = 10;
pub const CTA_STATS_EARLY_DROP: u16 = 11;
pub const CTA_STATS_ERROR: u16 = 12;
pub const CTA_STATS_SEARCH_RESTART: u16 = 13;
pub const CTA_STATS_CLASH_RESOLVE: u16 = 14;
pub const CTA_STATS_CHAIN_TOOLONG: u16 = 15;

pub const CTA_STATS_GLOBAL_UNSPEC: u8 = 0;
pub const CTA_STATS_GLOBAL_ENTRIES: u8 = 1;
pub const CTA_STATS_GLOBAL_MAX_ENTRIES: u8 = 2;

pub const CTA_STATS_EXP_UNSPEC: u8 = 0;
pub const CTA_STATS_EXP_NEW: u8 = 1;
pub const CTA_STATS_EXP_CREATE: u8 = 2;
pub const CTA_STATS_EXP_DELETE: u8 = 3;

pub const CTA_FILTER_UNSPEC: u8 = 0;
pub const CTA_FILTER_ORIG_FLAGS: u8 = 1;
pub const CTA_FILTER_REPLY_FLAGS: u8 = 2;
