//! Bitcoind's lightning backend.

use bitcoin_rpc::BitcoinRpc;
use bitcoin_amount::Amount;

use lightning;
use lightning::chain::chaininterface::ConfirmationTarget;

pub struct FeeEstimator {
    client: BitcoinRpc,
}

impl FeeEstimator {
    pub fn new(
        uri: String,
        user: Option<String>,
        pass: Option<String>
    ) -> FeeEstimator {
        FeeEstimator {
            client: BitcoinRpc::new(uri, user, pass),
        }
    }

    fn fallback_fee() -> Amount {
        Amount::from_btc(0.0001)
    }
}

impl lightning::chain::chaininterface::FeeEstimator for FeeEstimator {
	fn get_est_sat_per_1000_weight(
        &self,
        confirmation_target: ConfirmationTarget
    ) -> u64 {
        let blocks_to_wait = match confirmation_target {
            ConfirmationTarget::Background => 25,
            ConfirmationTarget::Normal => 6,
            ConfirmationTarget::HighPriority => 1,
        };

        let maybe_fee = self.client.estimatefee(blocks_to_wait);

        let fee_per_kb;
        match maybe_fee {
            Ok(fee) => fee_per_kb = fee,
            Err(_e) => {
                // TODO: log the error
                fee_per_kb = FeeEstimator::fallback_fee();
            }
        }

        assert!(fee_per_kb > Amount::zero());

        unimplemented!();
    }
}
