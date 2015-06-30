use std;
/// Structure represents prison.
/// Very simple wrapper around jail struct from libjail.
pub struct Prison {
    /// Id used in call syscalls. Can't be changes in runtime.
    /// Attention: this is not uniq jail identifier. This value might not be preserved during
    /// restarts
    id:         u32,
    /// Defines the version of the API in use. The only supported version right now is 2.
    pub version:    u32,
    /// Path to the root of prison. Can't be changed while prison is running.
    pub path:       String,
    /// Hostname of the jail. This Can be changed from inside of the prison.
    pub hostname:   String,
    /// Network address of epair _b_ side if any.
    pub ip4:        Option<std::net::Ipv4Addr>,
    /// IPv6 address of epair _b_ side if any.
    pub ip6:        Option<std::net::Ipv6Addr>
}
