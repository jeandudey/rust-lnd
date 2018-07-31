//! Bitcoind's lightning backend.

use std::cmp;

use bitcoin_rpc::{EstimateMode, BitcoinRpc};
use bitcoin_amount::{Amount, IntoBtc};

use lightning;
use lightning::chain::chaininterface::ConfirmationTarget;

pub struct FeeEstimator {
    client: BitcoinRpc,
    /// This is the minimum fee rate in sat/kB, of the backend node.
    min_feerate: Amount,
}

impl FeeEstimator {
    pub fn new(
        uri: String,
        user: Option<String>,
        pass: Option<String>,
    ) -> FeeEstimator {
        let client = BitcoinRpc::new(uri, user, pass);

        let networkinfo = client.getnetworkinfo().expect("couldn't call getnetworkinfo");
        let relayfee = Amount::from_btc(&networkinfo.relayfee);

        assert!(relayfee > Amount::zero());

        FeeEstimator {
            client: client,
            min_feerate: relayfee,
        }
    }
}

impl lightning::chain::chaininterface::FeeEstimator for FeeEstimator {
	fn get_est_sat_per_1000_weight(
        &self,
        confirmation_target: ConfirmationTarget
    ) -> u64 {
        let (blocks_to_wait, estimate_mode) = match confirmation_target {
            ConfirmationTarget::Background => (144, EstimateMode::Economical),
            ConfirmationTarget::Normal => (18, EstimateMode::Economical),
            ConfirmationTarget::HighPriority => (6, EstimateMode::Conservative),
        };

        let maybe_fee = self.client
            .estimatesmartfee(blocks_to_wait, estimate_mode)
            .map(|response| response.feerate.into_btc());

        let sat_per_kbyte = match maybe_fee {
            Ok(fee) => fee,
            Err(_) => self.min_feerate,
        };

        assert!(sat_per_kbyte > Amount::zero());

        let sat_per_kw = (sat_per_kbyte / Amount::from_sat(250)) +
                          Amount::from_sat(3);

        let sat_per_kw = cmp::max(sat_per_kw, Amount::from_sat(253));

        sat_per_kw.into_inner() as u64
    }
}
