use crate::commands::send::{Memo, SendArgs};
use crate::lib::{
    ledger_canister_id,
    signing::{sign_ingress_with_request_status_query, IngressWithRequestId},
    AnyhowResult,
};
use anyhow::{anyhow, bail, Context};
use candid::Encode;
use clap::Parser;
use ledger_canister::{ICPTs, TRANSACTION_FEE};

/// Signs an ICP transfer transaction.
#[derive(Default, Parser)]
pub struct TransferOpts {
    /// Destination account.
    pub to: String,

    /// Amount of ICPs to transfer (with up to 8 decimal digits after comma).
    #[clap(long, validator(icpts_amount_validator))]
    pub amount: String,

    /// Reference number, default is 0.
    #[clap(long, validator(memo_validator))]
    pub memo: Option<String>,

    /// Transaction fee, default is 10000 e8s.
    #[clap(long, validator(icpts_amount_validator))]
    pub fee: Option<String>,
}

pub fn exec(pem: &str, opts: TransferOpts) -> AnyhowResult<Vec<IngressWithRequestId>> {
    let amount = parse_icpts(&opts.amount).context("Cannot parse amount")?;
    let fee = opts.fee.map_or(Ok(TRANSACTION_FEE), |v| {
        parse_icpts(&v).context("Cannot parse fee")
    })?;
    let memo = Memo(
        opts.memo
            .unwrap_or_else(|| "0".to_string())
            .parse::<u64>()
            .context("Failed to parse memo as unsigned integer")?,
    );
    let to = opts.to;

    let args = Encode!(&SendArgs {
        memo,
        amount,
        fee,
        from_subaccount: None,
        to,
        created_at_time: None,
    })?;

    let msg = sign_ingress_with_request_status_query(pem, ledger_canister_id(), "send_dfx", args)?;
    Ok(vec![msg])
}

fn new_icps(icpt: u64, e8s: u64) -> AnyhowResult<ICPTs> {
    ICPTs::new(icpt, e8s)
        .map_err(|err| anyhow!(err))
        .context("Cannot create new ICPs")
}

fn parse_icpts(amount: &str) -> AnyhowResult<ICPTs> {
    let parse = |s: &str| {
        s.parse::<u64>()
            .context("Failed to parse ICPTs as unsigned integer")
    };
    match &amount.split('.').collect::<Vec<_>>().as_slice() {
        [icpts] => new_icps(parse(icpts)?, 0),
        [icpts, e8s] => {
            let mut e8s = e8s.to_string();
            while e8s.len() < 8 {
                e8s.push('0');
            }
            let e8s = &e8s[..8];
            new_icps(parse(icpts)?, parse(e8s)?)
        }
        _ => bail!("Cannot parse amount {}", amount),
    }
}

fn icpts_amount_validator(icpts: &str) -> AnyhowResult<()> {
    parse_icpts(icpts).map(|_| ())
}

fn memo_validator(memo: &str) -> Result<(), String> {
    if memo.parse::<u64>().is_ok() {
        return Ok(());
    }
    Err("Memo must be an unsigned integer".to_string())
}
