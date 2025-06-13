use core::fmt;

pub type PhaseMask = u8;

#[derive(Clone, Debug)]
pub enum Phase {
    Init = 0x01,
    RequestFilter = 0x02,
    UpstreamProxyFilter = 0x04,
    UpstreamPeerSelection = 0x08,
}

impl Phase {
    pub fn mask(&self) -> PhaseMask {
        self.clone() as PhaseMask
    }

    pub fn all_hook_mask() -> PhaseMask {
        let bits = Phase::Init.mask()
            | Phase::RequestFilter.mask()
            | Phase::UpstreamProxyFilter.mask()
            | Phase::UpstreamPeerSelection.mask();
        bits as PhaseMask
    }
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let phase_str = match self {
            Phase::Init => "init",
            Phase::RequestFilter => "request_filter",
            Phase::UpstreamProxyFilter => "upstream_proxy_filter",
            Phase::UpstreamPeerSelection => "upstream_peer_selection",
        };
        write!(f, "{}", phase_str)
    }
}

pub fn is_phase_enabed(phase_mask: PhaseMask, phase: &Phase) -> bool {
    (phase_mask & phase.mask()) == phase.mask()
}
