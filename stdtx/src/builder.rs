//! Builder for `StdTx` transactions which handles construction and signing.

use crate::{Error, Msg, Schema, Signer, StdFee, StdSignature, StdTx};
use serde_json::json;

/// [`StdTx`] transaction builder, which handles construction, signing, and
/// Amino serialization.
pub struct Builder {
    /// Schema which describes valid transaction types
    schema: Schema,

    /// Account number to include in transactions
    account_number: u64,

    /// Chain ID
    chain_id: String,

    /// Transaction signer
    signer: Box<Signer>,
}

impl Builder {
    /// Create a new transaction builder
    pub fn new(
        schema: Schema,
        account_number: u64,
        chain_id: impl Into<String>,
        signer: Box<Signer>,
    ) -> Self {
        Self {
            schema,
            account_number,
            chain_id: chain_id.into(),
            signer,
        }
    }

    /// Borrow this transaction builder's [`Schema`]
    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    /// Get this transaction builder's account number
    pub fn account_number(&self) -> u64 {
        self.account_number
    }

    /// Borrow this transaction builder's chain ID
    pub fn chain_id(&self) -> &str {
        &self.chain_id
    }

    /// Build and sign a transaction containing the given messages
    pub fn sign_tx(
        &self,
        sequence: u64,
        fee: StdFee,
        memo: &str,
        messages: &[Msg],
    ) -> Result<StdTx, Error> {
        let sign_msg = self.create_sign_msg(sequence, &fee, memo, messages);
        let signature = StdSignature::from(self.signer.try_sign(sign_msg.as_bytes())?);

        Ok(StdTx {
            msg: messages.iter().map(|msg| msg.to_amino_bytes()).collect(),
            fee: Some(fee),
            signatures: vec![signature],
            memo: memo.to_owned(),
        })
    }

    /// Build, sign, and encode a transaction in Amino format
    pub fn sign_amino_tx(
        &self,
        sequence: u64,
        fee: StdFee,
        memo: &str,
        messages: &[Msg],
    ) -> Result<Vec<u8>, Error> {
        let tx = self.sign_tx(sequence, fee, memo, messages)?;
        Ok(tx.to_amino_bytes(self.schema.namespace()))
    }

    /// Create the JSON message to sign for this transaction
    fn create_sign_msg(&self, sequence: u64, fee: &StdFee, memo: &str, messages: &[Msg]) -> String {
        let messages = messages
            .iter()
            .map(|msg| msg.to_json_value(&self.schema))
            .collect::<Vec<_>>();

        json!({
            "account_number": self.account_number.to_string(),
            "chain_id": self.chain_id,
            "fee": fee.to_json_value(),
            "memo": memo,
            "msgs": messages,
            "sequence": sequence.to_string()
        })
        .to_string()
    }
}
