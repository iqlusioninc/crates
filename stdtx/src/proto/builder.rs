//! Protocol Buffer-encoded transaction builder

// Includes code originally from ibc-rs:
// <https://github.com/informalsystems/ibc-rs>
// Copyright © 2020 Informal Systems Inc.
// Licensed under the Apache 2.0 license

use super::msg::Msg;
use crate::{Signature, VerifyKey};
use ecdsa::signature::Signer;
use eyre::Result;
use ibc_proto::cosmos::tx::v1beta1::{
    mode_info, AuthInfo, Fee, ModeInfo, SignDoc, SignerInfo, TxBody, TxRaw,
};
use tendermint::block;

/// Protocol Buffer-encoded transaction builder
pub struct Builder {
    /// Chain ID
    chain_id: String,

    /// Account number to include in transactions
    account_number: u64,
}

impl Builder {
    /// Create a new transaction builder
    pub fn new(chain_id: impl Into<String>, account_number: u64) -> Self {
        Self {
            chain_id: chain_id.into(),
            account_number,
        }
    }

    /// Borrow this transaction builder's chain ID
    pub fn chain_id(&self) -> &str {
        &self.chain_id
    }

    /// Get this transaction builder's account number
    pub fn account_number(&self) -> u64 {
        self.account_number
    }

    /// Build and sign a transaction containing the given messages
    pub fn sign_tx<S>(
        &self,
        signer: &S,
        sequence: u64,
        messages: &[Msg],
        fee: Fee,
        memo: impl Into<String>,
        timeout_height: block::Height,
    ) -> Result<Vec<u8>>
    where
        S: Signer<Signature>,
        VerifyKey: for<'a> From<&'a S>,
    {
        // Create TxBody
        let body = TxBody {
            messages: messages.iter().map(|msg| msg.0.clone()).collect(),
            memo: memo.into(),
            timeout_height: timeout_height.into(),
            extension_options: Default::default(),
            non_critical_extension_options: Default::default(),
        };

        // A protobuf serialization of a TxBody
        let mut body_buf = Vec::new();
        prost::Message::encode(&body, &mut body_buf).unwrap();

        let pk = VerifyKey::from(signer);
        let mut pk_buf = Vec::new();
        prost::Message::encode(&pk.to_bytes().to_vec(), &mut pk_buf).unwrap();

        // Create a MsgSend proto Any message
        // TODO(tarcieri): extract proper key type
        let pk_any = prost_types::Any {
            type_url: "/cosmos.crypto.secp256k1.PubKey".to_string(),
            value: Vec::from(&pk.to_bytes()[..]),
        };

        let single = mode_info::Single { mode: 1 };
        let mode = Some(ModeInfo {
            sum: Some(mode_info::Sum::Single(single)),
        });
        let signer_info = SignerInfo {
            public_key: Some(pk_any),
            mode_info: mode,
            sequence,
        };

        let auth_info = AuthInfo {
            signer_infos: vec![signer_info],
            fee: Some(fee),
        };

        // A protobuf serialization of a AuthInfo
        let mut auth_buf = Vec::new();
        prost::Message::encode(&auth_info, &mut auth_buf)?;

        let sign_doc = SignDoc {
            body_bytes: body_buf.clone(),
            auth_info_bytes: auth_buf.clone(),
            chain_id: self.chain_id.clone(),
            account_number: self.account_number,
        };

        // A protobuf serialization of a SignDoc
        let mut signdoc_buf = Vec::new();
        prost::Message::encode(&sign_doc, &mut signdoc_buf)?;

        // Sign the signdoc
        let signed = signer.sign(&signdoc_buf);

        let tx_raw = TxRaw {
            body_bytes: body_buf,
            auth_info_bytes: auth_buf,
            signatures: vec![signed.as_ref().to_vec()],
        };

        let mut txraw_buf = Vec::new();
        prost::Message::encode(&tx_raw, &mut txraw_buf)?;

        Ok(txraw_buf)
    }
}
