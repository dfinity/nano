use crate::commands::sign;
use crate::lib::environment::Environment;
use crate::lib::get_local_candid;
use crate::lib::nns_types::account_identifier::AccountIdentifier;
use crate::lib::nns_types::icpts::{ICPTs, TRANSACTION_FEE};
use crate::lib::nns_types::{Memo, SendArgs, LEDGER_CANISTER_ID};
use crate::lib::DfxResult;
use crate::lib::{get_candid_type, get_idl_string};
use anyhow::anyhow;
use candid::Encode;
use clap::Clap;
use ic_types::principal::Principal;
use std::str::FromStr;

const SEND_METHOD: &str = "send_dfx";

/// Transfer ICP from the user to the destination AccountIdentifier
#[derive(Default, Clap)]
pub struct TransferOpts {
    /// AccountIdentifier of transfer destination.
    pub to: String,

    /// ICPs to transfer to the destination AccountIdentifier
    /// Can be specified as a Decimal with the fractional portion up to 8 decimal places
    /// i.e. 100.012
    #[clap(long, validator(icpts_amount_validator))]
    pub amount: Option<String>,

    /// Specify ICP as a whole number, helpful for use in conjunction with `--e8s`
    #[clap(long, validator(e8s_validator), conflicts_with("amount"))]
    pub icp: Option<String>,

    /// Specify e8s as a whole number, helpful for use in conjunction with `--icp`
    #[clap(long, validator(e8s_validator), conflicts_with("amount"))]
    pub e8s: Option<String>,

    /// Specify a numeric memo for this transaction.
    #[clap(long, validator(memo_validator))]
    pub memo: Option<String>,

    /// Transaction fee, default is 10000 e8s.
    #[clap(long, validator(icpts_amount_validator))]
    pub fee: Option<String>,
}

pub async fn exec(env: &dyn Environment, opts: TransferOpts) -> DfxResult {
    let amount = get_icpts_from_args(opts.amount, opts.icp, opts.e8s)?;

    let fee = opts.fee.map_or(Ok(TRANSACTION_FEE), |v| {
        ICPTs::from_str(&v).map_err(|err| anyhow!(err))
    })?;

    // validated by memo_validator
    let memo = Memo(opts.memo.unwrap_or("0".to_string()).parse::<u64>().unwrap());

    let to = AccountIdentifier::from_str(&opts.to).map_err(|err| anyhow!(err))?;

    let canister_id = Principal::from_text(LEDGER_CANISTER_ID)?;

    let args = Encode!(&SendArgs {
        memo,
        amount,
        fee,
        from_subaccount: None,
        to,
        created_at_time: None,
    })?;

    let spec = get_local_candid(canister_id.clone());
    let method_type = spec.and_then(|spec| get_candid_type(spec, &SEND_METHOD));
    let argument = Some(get_idl_string(&args, "raw", &method_type)?);
    let opts = sign::SignOpts {
        canister_name: canister_id.to_string(),
        method_name: SEND_METHOD.to_string(),
        query: false,
        update: true,
        argument,
        r#type: Some("raw".to_string()),
        expire_after: "5m".to_string(),
    };
    sign::exec(env, opts).await
}

fn get_icpts_from_args(
    amount: Option<String>,
    icp: Option<String>,
    e8s: Option<String>,
) -> DfxResult<ICPTs> {
    if amount.is_none() {
        let icp = match icp {
            Some(s) => {
                // validated by e8s_validator
                let icps = s.parse::<u64>().unwrap();
                ICPTs::from_icpts(icps).map_err(|err| anyhow!(err))?
            }
            None => ICPTs::from_e8s(0),
        };
        let icp_from_e8s = match e8s {
            Some(s) => {
                // validated by e8s_validator
                let e8s = s.parse::<u64>().unwrap();
                ICPTs::from_e8s(e8s)
            }
            None => ICPTs::from_e8s(0),
        };
        let amount = icp + icp_from_e8s;
        Ok(amount.map_err(|err| anyhow!(err))?)
    } else {
        Ok(ICPTs::from_str(&amount.unwrap())
            .map_err(|err| anyhow!("Could not add ICPs and e8s: {}", err))?)
    }
}

fn e8s_validator(e8s: &str) -> Result<(), String> {
    if e8s.parse::<u64>().is_ok() {
        return Ok(());
    }
    Err("Must specify a non negative whole number.".to_string())
}

fn icpts_amount_validator(icpts: &str) -> Result<(), String> {
    ICPTs::from_str(icpts).map(|_| ())
}

fn memo_validator(memo: &str) -> Result<(), String> {
    if memo.parse::<u64>().is_ok() {
        return Ok(());
    }
    Err("Must specify a non negative whole number.".to_string())
}
