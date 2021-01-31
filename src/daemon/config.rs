// MyCitadel: node, wallet library & command-line tool
// Written in 2021 by
//     Dr. Maxim Orlovsky <orlovsky@mycitadel.io>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the AGPL License
// along with this software.
// If not, see <https://www.gnu.org/licenses/agpl-3.0-standalone.html>.

use std::path::PathBuf;

use internet2::zmqsocket::ZmqSocketAddr;
use lnpbp::Chain;
use microservices::FileFormat;

#[cfg(feature = "shell")]
use super::Opts;
use crate::storage;

/// Final configuration resulting from data contained in config file environment
/// variables and command-line options. For security reasons node key is kept
/// separately.
#[derive(Clone, PartialEq, Eq, Debug, Display)]
#[display(Debug)]
pub struct Config {
    /// Bitcoin blockchain to use (mainnet, testnet, signet, liquid etc)
    pub chain: Chain,

    /// ZMQ socket for RPC API
    pub rpc_endpoint: ZmqSocketAddr,

    /// RGB20 ZMQ RPC API endpoint
    pub rgb20_endpoint: ZmqSocketAddr,

    /// Data location
    pub data_dir: PathBuf,

    /// Verbosity level
    pub verbose: u8,

    /// Electrum server connection string
    pub electrum_server: String,
}

impl Config {
    pub fn storage_conf(&self) -> storage::file::FileConfig {
        let format = FileFormat::Yaml;

        let mut data_filename = self.data_dir.clone();
        data_filename.push("citadel");
        data_filename.set_extension(format.extension());

        storage::file::FileConfig {
            location: data_filename.to_string_lossy().to_string(),
            format,
        }
    }
}

#[cfg(feature = "shell")]
impl From<Opts> for Config {
    fn from(opts: Opts) -> Self {
        Config {
            chain: opts.chain,
            data_dir: opts.data_dir,
            rpc_endpoint: opts.shared.rpc_endpoint,
            rgb20_endpoint: opts.rgb20_endpoint,
            verbose: opts.shared.verbose,
            electrum_server: opts.electrum_server,
        }
    }
}
