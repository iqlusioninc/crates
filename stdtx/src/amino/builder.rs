//! Builder for `StdTx` transactions which handles construction and signing.

use super::{Msg, Schema, StdFee, StdSignature, StdTx};
use crate::Signer;
use eyre::Result;
use serde_json::json;

/// [`StdTx`] transaction builder, which handles construction, signing, and
/// Amino serialization.
pub struct Builder {
    /// Schema which describes valid transaction types
    schema: Schema,

    /// Chain ID
    chain_id: String,

    /// Account number to include in transactions
    account_number: u64,
}

impl Builder {
    /// Create a new transaction builder
    pub fn new(schema: Schema, chain_id: impl Into<String>, account_number: u64) -> Self {
        Self {
            schema,
            chain_id: chain_id.into(),
            account_number,
        }
    }

    /// Borrow this transaction builder's [`Schema`]
    pub fn schema(&self) -> &Schema {
        &self.schema
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
    pub fn sign_tx(
        &self,
        signer: &Signer,
        sequence: u64,
        fee: StdFee,
        memo: &str,
        messages: &[Msg],
    ) -> Result<StdTx> {
        let sign_msg = self.create_sign_msg(sequence, &fee, memo, messages);
        let signature = StdSignature::from(signer.try_sign(sign_msg.as_bytes())?);
        Ok(StdTx::new(messages, fee, vec![signature], memo))
    }

    /// Build, sign, and encode a transaction in Amino format
    pub fn sign_amino_tx(
        &self,
        signer: &Signer,
        sequence: u64,
        fee: StdFee,
        memo: &str,
        messages: &[Msg],
    ) -> Result<Vec<u8>> {
        let tx = self.sign_tx(signer, sequence, fee, memo, messages)?;
        Ok(tx.to_amino_bytes(self.schema.namespace()))
    }

    /// Create the JSON message to sign for this transaction
    pub fn create_sign_json(
        &self,
        sequence: u64,
        fee: &StdFee,
        memo: &str,
        messages: &[Msg],
    ) -> serde_json::Value {
        let messages = messages
            .iter()
            .map(|msg| msg.to_json_value(&self.schema))
            .collect::<Vec<_>>();

        json!({
            "account_number": self.account_number.to_string(),
            "chain_id": self.chain_id,
            "fee": fee,
            "memo": memo,
            "msgs": messages,
            "sequence": sequence.to_string()
        })
    }

    /// Create the serialized JSON string to sign for this transaction
    pub fn create_sign_msg(
        &self,
        sequence: u64,
        fee: &StdFee,
        memo: &str,
        messages: &[Msg],
    ) -> String {
        self.create_sign_json(sequence, fee, memo, messages)
            .to_string()
    }
}
