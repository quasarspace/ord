use super::error::ApiError;
use super::*;
use axum::Json;

const ERR_TICK_LENGTH: &str = "tick must be 4 bytes length";

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickInfo {
  pub tick: String,
  pub inscription_id: String,
  pub supply: String,
  pub limit_per_mint: String,
  pub minted: String,
  pub decimal: u64,
  pub deploy_by: String,
  pub txid: String,
  pub deploy_height: u64,
  pub deploy_blocktime: u64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
  pub tick: String,
  pub available_balance: String,
  pub transferable_balance: String,
  pub overall_balance: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxEvents {
  pub events: Vec<TxEvent>,
  pub txid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event")]
#[serde(rename_all = "camelCase")]
pub enum TxEvent {
  Deploy(DeployEvent),
  Mint(MintEvent),
  InscribeTransfer(InscribeTransferEvent),
  Transfer(TransferEvent),
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeployEvent {
  pub tick: String,
  pub inscription_id: String,
  pub supply: String,
  pub limit_per_mint: String,
  pub decimal: u64,
  pub deploy_by: String,
  pub valid: bool,
  pub msg: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MintEvent {
  pub tick: String,
  pub inscription_id: String,
  pub amount: String,
  pub to: String,
  pub valid: bool,
  pub msg: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InscribeTransferEvent {
  pub tick: String,
  pub inscription_id: String,
  pub amount: String,
  pub owner: String,
  pub valid: bool,
  pub msg: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferEvent {
  pub tick: String,
  pub amount: String,
  pub from: String,
  pub to: String,
  pub valid: bool,
  pub msg: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockEvents {
  pub block: Vec<TxEvents>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferableInscriptions {
  pub inscriptions: Vec<TransferableInscription>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferableInscription {
  pub id: String,
  pub amount: String,
}

pub(crate) async fn brc20_tick_info(
  Extension(index): Extension<Arc<Index>>,
  Path(tick): Path<String>,
) -> Json<ApiResponse<TickInfo>> {
  if tick.as_bytes().len() != 4 {
    return Json(ApiResponse::api_err(&ApiError::BadRequest(
      ERR_TICK_LENGTH.to_string(),
    )));
  }
  let tick = tick.to_lowercase();

  let tick_info = match index.brc20_get_tick_info(&tick) {
    Ok(tick_info) => tick_info,
    Err(err) => return Json(ApiResponse::api_err(&ApiError::Internal(err.to_string()))),
  };

  if tick_info.is_none() {
    return Json(ApiResponse::api_err(&ApiError::not_found("tick not found")));
  }

  let tick_info = tick_info.unwrap();

  // if tick_info.tick != tick {
  //   return Json(ApiResponse::api_err(&ApiError::internal("db: not match")));
  // }

  Json(ApiResponse::ok(TickInfo {
    tick,
    inscription_id: tick_info.inscription_id.to_string(),
    supply: tick_info.supply.to_string(),
    limit_per_mint: tick_info.limit_per_mint.to_string(),
    minted: tick_info.minted.to_string(),
    decimal: tick_info.decimal as u64,
    deploy_by: tick_info.deploy_by.to_string(),
    txid: tick_info.inscription_id.txid.to_string(),
    deploy_height: tick_info.deployed_number,
    deploy_blocktime: 1234,
  }))
}

pub(crate) async fn brc20_balance(
  Extension(index): Extension<Arc<Index>>,
  Path((tick, address)): Path<(String, String)>,
) -> Json<ApiResponse<Balance>> {
  if tick.as_bytes().len() != 4 {
    return Json(ApiResponse::api_err(&ApiError::BadRequest(
      ERR_TICK_LENGTH.to_string(),
    )));
  }
  let tick = tick.to_lowercase();

  let address: bitcoin::Address = match address.parse() {
    Ok(address) => address,
    Err(err) => return Json(ApiResponse::api_err(&ApiError::BadRequest(err.to_string()))),
  };

  let balance = match index.brc20_get_balance_by_address(&tick, &address) {
    Ok(balance) => balance,
    Err(err) => return Json(ApiResponse::api_err(&ApiError::Internal(err.to_string()))),
  };

  if balance.is_none() {
    return Json(ApiResponse::api_err(&ApiError::not_found(
      "balance not found",
    )));
  }

  let balance = balance.unwrap();

  let available_balance = balance.overall_balance - balance.transferable_balance;
  if available_balance > balance.overall_balance {
    return Json(ApiResponse::api_err(&ApiError::internal("balance error")));
  }

  Json(ApiResponse::ok(Balance {
    tick,
    available_balance: available_balance.to_string(),
    transferable_balance: balance.transferable_balance.to_string(),
    overall_balance: balance.overall_balance.to_string(),
  }))
}

pub(crate) async fn brc20_tx_events(
  Extension(index): Extension<Arc<Index>>,
  Path(txid): Path<String>,
) -> Json<ApiResponse<TxEvents>> {
  todo!();
}

pub(crate) async fn brc20_block_events(
  Extension(index): Extension<Arc<Index>>,
  Path(block_hash): Path<String>,
) -> Json<ApiResponse<BlockEvents>> {
  todo!();
}

pub(crate) async fn brc20_transferable(
  Extension(index): Extension<Arc<Index>>,
  Path((tick, addr)): Path<(String, String)>,
) -> Json<ApiResponse<TransferableInscriptions>> {
  todo!();
}
