use crate::commands::{
    send::{Memo, SendArgs},
    sign::sign_ingress_with_request_status_query,
};
use crate::lib::{ledger_canister_id, sign::signed_message::IngressWithRequestId, AnyhowResult};
use anyhow::anyhow;
use candid::Encode;
use clap::Clap;
use ledger_canister::{AccountIdentifier, ICPTs, TRANSACTION_FEE};
use std::str::FromStr;

/// Signs an ICP transfer transaction.
#[derive(Default, Clap)]
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

pub async fn exec(
    pem: &Option<String>,
    opts: TransferOpts,
) -> AnyhowResult<Vec<IngressWithRequestId>> {
    let amount =
        parse_icpts(&opts.amount).map_err(|err| anyhow!("Could not add ICPs and e8s: {}", err))?;
    let fee = opts.fee.map_or(Ok(TRANSACTION_FEE), |v| {
        parse_icpts(&v).map_err(|err| anyhow!(err))
    })?;
    let memo = Memo(
        opts.memo
            .unwrap_or_else(|| "0".to_string())
            .parse::<u64>()
            .unwrap(),
    );
    let to = AccountIdentifier::from_str(&opts.to).map_err(|err| anyhow!(err))?;

    let args = Encode!(&SendArgs {
        memo,
        amount,
        fee,
        from_subaccount: None,
        to,
        created_at_time: None,
    })?;

    let msg =
        sign_ingress_with_request_status_query(pem, ledger_canister_id(), "send_dfx", args).await?;
    Ok(vec![msg])
}

fn parse_icpts(amount: &str) -> Result<ICPTs, String> {
    let mut it = amount.split('.');
    let icpts = it
        .next()
        .unwrap_or("0")
        .parse::<u64>()
        .map_err(|err| format!("Couldn't parse icpts: {:?}", err))?;

    let mut e8s = it.next().unwrap_or("0").to_string();
    while e8s.len() < 8 {
        e8s.push('0');
    }
    let e8s = e8s
        .parse::<u64>()
        .map_err(|err| format!("Couldn't parse e8s: {:?}", err))?;

    ICPTs::new(icpts, e8s)
}

fn icpts_amount_validator(icpts: &str) -> Result<(), String> {
    parse_icpts(icpts).map(|_| ())
}

fn memo_validator(memo: &str) -> Result<(), String> {
    if memo.parse::<u64>().is_ok() {
        return Ok(());
    }
    Err("Memo must be an unsigned integer".to_string())
}
