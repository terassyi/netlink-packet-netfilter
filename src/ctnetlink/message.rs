// SPDX-License-Identifier: MIT

use netlink_packet_utils::{
    nla::DefaultNla, DecodeError, Emitable, Parseable, ParseableParametrized,
};

use crate::{
    buffer::NetfilterBuffer,
    constants::{
        IPCTNL_MSG_CT_DELETE, IPCTNL_MSG_CT_GET, IPCTNL_MSG_CT_GET_CTRZERO,
        IPCTNL_MSG_CT_GET_DYING, IPCTNL_MSG_CT_GET_STATS,
        IPCTNL_MSG_CT_GET_STATS_CPU, IPCTNL_MSG_CT_GET_UNCONFIRMED,
        IPCTNL_MSG_CT_NEW, NFNL_SUBSYS_CTNETLINK,
    },
};

use super::nlas::{flow::nla::FlowNla, stat::nla::StatNla};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CtNetlinkMessage {
    New(Vec<FlowNla>),
    Get(Option<Vec<FlowNla>>),
    Delete(Vec<FlowNla>),
    GetCrtZero(Option<Vec<FlowNla>>),
    GetStatsCPU(Option<Vec<StatNla>>),
    GetStats(Option<Vec<StatNla>>),
    GetDying(Option<Vec<FlowNla>>),
    GetUnconfirmed(Option<Vec<FlowNla>>),
    Other {
        message_type: u8,
        nlas: Vec<DefaultNla>,
    },
}

impl CtNetlinkMessage {
    pub const SUBSYS: u8 = NFNL_SUBSYS_CTNETLINK;

    pub fn message_type(&self) -> u8 {
        match self {
            CtNetlinkMessage::New(_) => IPCTNL_MSG_CT_NEW,
            CtNetlinkMessage::Get(_) => IPCTNL_MSG_CT_GET,
            CtNetlinkMessage::Delete(_) => IPCTNL_MSG_CT_DELETE,
            CtNetlinkMessage::GetCrtZero(_) => IPCTNL_MSG_CT_GET_CTRZERO,
            CtNetlinkMessage::GetStatsCPU(_) => IPCTNL_MSG_CT_GET_STATS_CPU,
            CtNetlinkMessage::GetStats(_) => IPCTNL_MSG_CT_GET_STATS,
            CtNetlinkMessage::GetDying(_) => IPCTNL_MSG_CT_GET_DYING,
            CtNetlinkMessage::GetUnconfirmed(_) => {
                IPCTNL_MSG_CT_GET_UNCONFIRMED
            }
            CtNetlinkMessage::Other { message_type, .. } => *message_type,
        }
    }
}

impl Emitable for CtNetlinkMessage {
    fn buffer_len(&self) -> usize {
        match self {
            CtNetlinkMessage::New(nlas) => nlas.as_slice().buffer_len(),
            CtNetlinkMessage::Get(nlas) => match nlas {
                Some(nlas) => nlas.as_slice().buffer_len(),
                None => 0,
            },
            CtNetlinkMessage::Delete(nlas) => nlas.as_slice().buffer_len(),
            CtNetlinkMessage::GetCrtZero(nlas) => match nlas {
                Some(nlas) => nlas.as_slice().buffer_len(),
                None => 0,
            },
            CtNetlinkMessage::GetStatsCPU(nlas) => match nlas {
                Some(nlas) => nlas.as_slice().buffer_len(),
                None => 0,
            },
            CtNetlinkMessage::GetStats(nlas) => match nlas {
                Some(nlas) => nlas.as_slice().buffer_len(),
                None => 0,
            },
            CtNetlinkMessage::GetDying(nlas) => match nlas {
                Some(nlas) => nlas.as_slice().buffer_len(),
                None => 0,
            },
            CtNetlinkMessage::GetUnconfirmed(nlas) => match nlas {
                Some(nlas) => nlas.as_slice().buffer_len(),
                None => 0,
            },
            CtNetlinkMessage::Other { nlas, .. } => {
                nlas.as_slice().buffer_len()
            }
        }
    }

    fn emit(&self, buffer: &mut [u8]) {
        match self {
            CtNetlinkMessage::New(nlas) => nlas.as_slice().emit(buffer),
            CtNetlinkMessage::Get(nlas) => {
                if let Some(nlas) = nlas {
                    nlas.as_slice().emit(buffer);
                }
            }
            CtNetlinkMessage::GetCrtZero(nlas) => {
                if let Some(nlas) = nlas {
                    nlas.as_slice().emit(buffer);
                }
            }
            CtNetlinkMessage::Delete(nlas) => nlas.as_slice().emit(buffer),
            CtNetlinkMessage::GetStatsCPU(nlas) => {
                if let Some(nlas) = nlas {
                    nlas.as_slice().emit(buffer)
                }
            }
            CtNetlinkMessage::GetStats(nlas) => {
                if let Some(nlas) = nlas {
                    nlas.as_slice().emit(buffer)
                }
            }
            CtNetlinkMessage::GetDying(nlas) => {
                if let Some(nlas) = nlas {
                    nlas.as_slice().emit(buffer);
                }
            }
            CtNetlinkMessage::GetUnconfirmed(nlas) => {
                if let Some(nlas) = nlas {
                    nlas.as_slice().emit(buffer);
                }
            }
            CtNetlinkMessage::Other { nlas, .. } => {
                nlas.as_slice().emit(buffer)
            }
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized>
    ParseableParametrized<NetfilterBuffer<&'a T>, u8> for CtNetlinkMessage
{
    fn parse_with_param(
        buf: &NetfilterBuffer<&'a T>,
        message_type: u8,
    ) -> Result<Self, DecodeError> {
        Ok(match message_type {
            IPCTNL_MSG_CT_NEW => {
                let nlas =
                    buf.parse_all_nlas(|nla_buf| FlowNla::parse(&nla_buf))?;
                CtNetlinkMessage::New(nlas)
            }
            IPCTNL_MSG_CT_GET => {
                if buf.payload().is_empty() {
                    CtNetlinkMessage::Get(None)
                } else {
                    let nlas =
                        buf.parse_all_nlas(|nla_buf| FlowNla::parse(&nla_buf))?;
                    CtNetlinkMessage::Get(Some(nlas))
                }
            }
            IPCTNL_MSG_CT_DELETE => {
                let nlas =
                    buf.parse_all_nlas(|nla_buf| FlowNla::parse(&nla_buf))?;
                CtNetlinkMessage::Delete(nlas)
            }
            IPCTNL_MSG_CT_GET_CTRZERO => {
                if buf.payload().is_empty() {
                    CtNetlinkMessage::GetCrtZero(None)
                } else {
                    let nlas =
                        buf.parse_all_nlas(|nla_buf| FlowNla::parse(&nla_buf))?;
                    CtNetlinkMessage::GetCrtZero(Some(nlas))
                }
            }
            IPCTNL_MSG_CT_GET_STATS_CPU => {
                if buf.payload().is_empty() {
                    CtNetlinkMessage::GetStatsCPU(None)
                } else {
                    let nlas =
                        buf.parse_all_nlas(|nla_buf| StatNla::parse(&nla_buf))?;
                    CtNetlinkMessage::GetStatsCPU(Some(nlas))
                }
            }
            IPCTNL_MSG_CT_GET_STATS => {
                if buf.payload().is_empty() {
                    CtNetlinkMessage::GetStats(None)
                } else {
                    let nlas =
                        buf.parse_all_nlas(|nla_buf| StatNla::parse(&nla_buf))?;
                    CtNetlinkMessage::GetStats(Some(nlas))
                }
            }
            IPCTNL_MSG_CT_GET_DYING => {
                if buf.payload().is_empty() {
                    CtNetlinkMessage::GetDying(None)
                } else {
                    let nlas =
                        buf.parse_all_nlas(|nla_buf| FlowNla::parse(&nla_buf))?;
                    CtNetlinkMessage::GetDying(Some(nlas))
                }
            }
            IPCTNL_MSG_CT_GET_UNCONFIRMED => {
                if buf.payload().is_empty() {
                    CtNetlinkMessage::GetUnconfirmed(None)
                } else {
                    let nlas =
                        buf.parse_all_nlas(|nla_buf| FlowNla::parse(&nla_buf))?;
                    CtNetlinkMessage::GetUnconfirmed(Some(nlas))
                }
            }
            _ => CtNetlinkMessage::Other {
                message_type,
                nlas: buf.default_nlas()?,
            },
        })
    }
}
