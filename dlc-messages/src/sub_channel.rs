//!
//!

use bitcoin::Script;
use secp256k1_zkp::{ecdsa::Signature, EcdsaAdaptorSignature, PublicKey, SecretKey};

use crate::ser_impls::{read_ecdsa_adaptor_signature, write_ecdsa_adaptor_signature};
use crate::{contract_msgs::ContractInfo, CetAdaptorSignatures};
use lightning::ln::msgs::DecodeError;
use lightning::util::ser::{Readable, Writeable, Writer};

///
#[derive(Clone, Debug)]
pub enum SubChannelMessage {
    ///
    Request(SubChannelOffer),
    ///
    Accept(SubChannelAccept),
    ///
    Confirm(SubChannelConfirm),
    ///
    Finalize(SubChannelFinalize),
    ///
    CloseOffer(SubChannelCloseOffer),
    ///
    CloseAccept(SubChannelCloseAccept),
    ///
    CloseConfirm(SubChannelCloseConfirm),
    ///
    CloseFinalize(SubChannelCloseFinalize),
    ///
    CloseReject(SubChannelCloseReject),
}

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
///
pub struct SubChannelOffer {
    ///
    pub channel_id: [u8; 32],
    /// The base point that will be used by the offer party for revocation.
    pub revocation_basepoint: PublicKey,
    /// The base point that will be used by the offer party for generating
    /// adaptor signatures to revocable transactions.
    pub publish_basepoint: PublicKey,
    /// The base point that will be used by the offer party in the 2 of 2 output
    /// of buffer transactions.
    pub own_basepoint: PublicKey,
    ///
    pub next_per_split_point: PublicKey,
    // TODO(tibo): Channel related fields would be nice in a TLV to separate concerns.
    ///
    pub contract_info: ContractInfo,
    /// The base point that will be used by the offer party for revocation.
    pub channel_revocation_basepoint: PublicKey,
    /// The base point that will be used by the offer party for generating
    /// adaptor signatures to revocable transactions.
    pub channel_publish_basepoint: PublicKey,
    /// The base point that will be used by the offer party in the 2 of 2 output
    /// of buffer transactions.
    pub channel_own_basepoint: PublicKey,
    ///
    pub channel_first_per_update_point: PublicKey,
    /// Script used by the offer party to receive their payout on channel close.
    pub payout_spk: Script,
    /// Serial id used to order outputs.
    pub payout_serial_id: u64,
    /// The collateral input by the offer party in the channel.
    pub offer_collateral: u64,
    /// Lock time for the CETs.
    pub cet_locktime: u32,
    /// Lock time for the refund transaction.
    pub refund_locktime: u32,
    /// The nSequence value to use for the CETs.
    pub cet_nsequence: u32,
    ///
    pub fee_rate_per_vbyte: u64,
}

impl_dlc_writeable!(
    SubChannelOffer, {
    (channel_id, writeable),
    (revocation_basepoint, writeable),
    (publish_basepoint, writeable),
    (own_basepoint, writeable),
    (next_per_split_point, writeable),
    (contract_info, writeable),
    (channel_revocation_basepoint, writeable),
    (channel_publish_basepoint, writeable),
    (channel_own_basepoint, writeable),
    (channel_first_per_update_point, writeable),
    (payout_spk, writeable),
    (payout_serial_id, writeable),
    (offer_collateral, writeable),
    (cet_locktime, writeable),
    (refund_locktime, writeable),
    (cet_nsequence, writeable),
    (fee_rate_per_vbyte, writeable)
    }
);

#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
///
pub struct SubChannelInfo {
    ///
    pub sender_satoshi: u64,
    ///
    pub receiver_satoshi: u64,
}

impl_dlc_writeable!(SubChannelInfo, {(sender_satoshi, writeable), (receiver_satoshi, writeable)});

///
#[derive(Debug, Clone)]
pub struct SubChannelAccept {
    ///
    pub channel_id: [u8; 32],
    /// The base point that will be used by the offer party for revocation.
    pub revocation_basepoint: PublicKey,
    /// The base point that will be used by the offer party for generating
    /// adaptor signatures to revocable transactions.
    pub publish_basepoint: PublicKey,
    /// The base point that will be used by the offer party in the 2 of 2 output
    /// of buffer transactions.
    pub own_basepoint: PublicKey,
    ///
    pub split_adaptor_signature: EcdsaAdaptorSignature,
    ///
    pub commit_signature: Signature,
    ///
    pub htlc_signatures: Vec<Signature>,
    ///
    pub first_per_split_point: PublicKey,
    /// The base point that will be used by the offer party for revocation.
    pub channel_revocation_basepoint: PublicKey,
    /// The base point that will be used by the offer party for generating
    /// adaptor signatures to revocable transactions.
    pub channel_publish_basepoint: PublicKey,
    /// The base point that will be used by the offer party in the 2 of 2 output
    /// of buffer transactions.
    pub channel_own_basepoint: PublicKey,
    /// The adaptor signatures for all CETs generated by the accept party.
    pub cet_adaptor_signatures: CetAdaptorSignatures,
    /// The adaptor signature for the buffer transaction generated by the accept
    /// party.
    pub buffer_adaptor_signature: EcdsaAdaptorSignature,
    /// The refund signature generated by the accept party.
    pub refund_signature: Signature,
    ///
    pub ln_glue_signature: Signature,
    ///
    pub first_per_update_point: PublicKey,
    ///
    pub payout_spk: Script,
    ///
    pub payout_serial_id: u64,
}

impl_dlc_writeable!(
    SubChannelAccept, {
    (channel_id, writeable),
    (revocation_basepoint, writeable),
    (publish_basepoint, writeable),
    (own_basepoint, writeable),
    (split_adaptor_signature, {cb_writeable, write_ecdsa_adaptor_signature, read_ecdsa_adaptor_signature}),
    (commit_signature, writeable),
    (htlc_signatures, writeable),
    (first_per_split_point, writeable),
    (channel_revocation_basepoint, writeable),
    (channel_publish_basepoint, writeable),
    (channel_own_basepoint, writeable),
    (cet_adaptor_signatures, writeable),
    (buffer_adaptor_signature,  {cb_writeable, write_ecdsa_adaptor_signature, read_ecdsa_adaptor_signature}),
    (refund_signature, writeable),
    (ln_glue_signature, writeable),
    (first_per_update_point, writeable),
    (payout_spk, writeable),
    (payout_serial_id, writeable)
    }
);

///
#[derive(Clone, Debug)]
pub struct SubChannelConfirm {
    ///
    pub channel_id: [u8; 32],
    ///
    pub per_commitment_secret: SecretKey,
    ///
    pub next_per_commitment_point: PublicKey,
    ///
    pub split_adaptor_signature: EcdsaAdaptorSignature,
    ///
    pub commit_signature: Signature,
    ///
    pub htlc_signatures: Vec<Signature>,
    ///
    pub cet_adaptor_signatures: CetAdaptorSignatures,
    /// The adaptor signature for the buffer transaction generated by the offer
    /// party.
    pub buffer_adaptor_signature: EcdsaAdaptorSignature,
    /// The refund signature generated by the offer party.
    pub refund_signature: Signature,
    ///
    pub ln_glue_signature: Signature,
}

impl_dlc_writeable!(SubChannelConfirm, {
    (channel_id, writeable),
    (per_commitment_secret, writeable),
    (next_per_commitment_point, writeable),
    (split_adaptor_signature, {cb_writeable, write_ecdsa_adaptor_signature, read_ecdsa_adaptor_signature}),
    (commit_signature, writeable),
    (htlc_signatures, writeable),
    (cet_adaptor_signatures, writeable),
    (buffer_adaptor_signature, {cb_writeable, write_ecdsa_adaptor_signature, read_ecdsa_adaptor_signature}),
    (refund_signature, writeable),
    (ln_glue_signature, writeable)
});

///
#[derive(Clone, Debug)]
pub struct SubChannelFinalize {
    ///
    pub channel_id: [u8; 32],
    ///
    pub per_commitment_secret: SecretKey,
    ///
    pub next_per_commitment_point: PublicKey,
}

impl_dlc_writeable!(SubChannelFinalize, {
    (channel_id, writeable),
    (per_commitment_secret, writeable),
    (next_per_commitment_point, writeable)

});

///
#[derive(Clone, Debug)]
pub struct SubChannelCloseOffer {
    ///
    pub channel_id: [u8; 32],
    ///
    pub accept_balance: u64,
}

impl_dlc_writeable!(SubChannelCloseOffer, {
    (channel_id, writeable),
    (accept_balance, writeable)
});

///
#[derive(Clone, Debug)]
pub struct SubChannelCloseAccept {
    ///
    pub channel_id: [u8; 32],
    ///
    pub commit_signature: Signature,
    ///
    pub htlc_signatures: Vec<Signature>,
}

impl_dlc_writeable!(SubChannelCloseAccept, {
    (channel_id, writeable),
    (commit_signature, writeable),
    (htlc_signatures, writeable)
});

///
#[derive(Clone, Debug)]
pub struct SubChannelCloseConfirm {
    ///
    pub channel_id: [u8; 32],
    ///
    pub commit_signature: Signature,
    ///
    pub htlc_signatures: Vec<Signature>,
    ///
    pub split_revocation_secret: SecretKey,
    ///
    pub commit_revocation_secret: SecretKey,
    ///
    pub next_per_commitment_point: PublicKey,
}

impl_dlc_writeable!(SubChannelCloseConfirm, {
    (channel_id, writeable),
    (commit_signature, writeable),
    (htlc_signatures, writeable),
    (split_revocation_secret, writeable),
    (commit_revocation_secret, writeable),
    (next_per_commitment_point, writeable)
});

///
#[derive(Clone, Debug)]
pub struct SubChannelCloseFinalize {
    ///
    pub channel_id: [u8; 32],
    ///
    pub split_revocation_secret: SecretKey,
    ///
    pub commit_revocation_secret: SecretKey,
    ///
    pub next_per_commitment_point: PublicKey,
}

impl_dlc_writeable!(SubChannelCloseFinalize, {
    (channel_id, writeable),
    (split_revocation_secret, writeable),
    (commit_revocation_secret, writeable),
    (next_per_commitment_point, writeable)
});

///
#[derive(Clone, Debug)]
pub struct SubChannelCloseReject {
    ///
    pub channel_id: [u8; 32],
}

impl_dlc_writeable!(SubChannelCloseReject, { (channel_id, writeable) });
