use crate::lib::{get_account_id, get_identity, AnyhowResult, AuthInfo};
use anyhow::anyhow;
use candid::Principal;
use clap::Parser;
use ledger_canister::AccountIdentifier;

#[derive(Parser)]
pub struct PublicOpts {
    /// Principal for which to get the account_id.
    #[clap(long)]
    principal_id: Option<String>,
}

/// Prints the account and the principal ids.
pub fn exec(auth: &AuthInfo, opts: PublicOpts) -> AnyhowResult {
    let (principal_id, account_id) = get_public_ids(auth, opts)?;
    println!("Principal id: {}", principal_id.to_text());
    println!("Account id: {}", account_id);
    Ok(())
}

/// Returns the account id and the principal id if the private key was provided.
fn get_public_ids(
    auth: &AuthInfo,
    opts: PublicOpts,
) -> AnyhowResult<(Principal, AccountIdentifier)> {
    match opts.principal_id {
        Some(principal_id) => {
            let principal_id = Principal::from_text(principal_id)?;
            Ok((principal_id, get_account_id(principal_id)?))
        }
        None => {
            if let AuthInfo::NoAuth = auth {
                Err(anyhow!(
                    "public-ids cannot be used without specifying a private key"
                ))
            } else {
                get_ids(auth)
            }
        }
    }
}

/// Returns the account id and the principal id if the private key was provided.
pub fn get_ids(auth: &AuthInfo) -> AnyhowResult<(Principal, AccountIdentifier)> {
    let principal_id = get_identity(auth)?.sender().map_err(|e| anyhow!(e))?;
    Ok((principal_id, get_account_id(principal_id)?))
}
