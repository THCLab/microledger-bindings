use keri::{prefix::{BasicPrefix, CesrPrimitive}, keys::PublicKey};
use nontransferable::{NontransferableIdentifier, NontransferableSignature, NontransferableVerifier};
use microledger::{
  block::Block,
  microledger::MicroLedger,
  seal_bundle::{SealBundle, SealData},
  Encode,
};
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use std::sync::Arc;

mod nontransferable;

  #[napi]
  pub fn into_identifier(pk: Buffer) -> String {
    let bp = BasicPrefix::Ed25519NT(PublicKey::new(pk.to_vec()));
    bp.to_str()
  }

#[napi(js_name = "Microledger")]
struct JsMicroledger {
  micro: MicroLedger<NontransferableSignature, NontransferableVerifier, NontransferableIdentifier>,
}

#[napi]
impl JsMicroledger {
  #[napi(constructor)]
  pub fn new() -> Self {
    let validator = Arc::new(NontransferableVerifier);
    let microledger = MicroLedger::new(validator);
    JsMicroledger { micro: microledger }
  }

  #[napi]
  pub fn pre_anchor_block(
    &self,
    attachments: Vec<String>,
    identifiers: Vec<String>,
  ) -> napi::Result<String> {
    let mut seal_bundle = SealBundle::new();
    for seal in attachments {
      seal_bundle = seal_bundle.attach(SealData::AttachedData(seal));
    }

    let mut controlling_ids: Vec<NontransferableIdentifier> = vec![];
    for i in identifiers {
      let id: BasicPrefix = i.parse().unwrap();

      controlling_ids.push(NontransferableIdentifier(id));
    }
    let block = self
      .micro
      .pre_anchor_block(controlling_ids, &seal_bundle)
      .unwrap();
    let block_str = String::from_utf8(block.encode().unwrap()).unwrap();

    Ok(block_str)
  }

  #[napi]
  pub fn anchor_block(&mut self, block: String, identifier: String, signature: Buffer) -> napi::Result<String> {
    let block: Block<NontransferableIdentifier> = serde_json::from_str(&block).unwrap();

    let signature = NontransferableSignature::new(identifier, signature.to_vec());
    let signed_block = block.to_signed_block(vec![signature]);

    self.micro.anchor(signed_block.clone()).unwrap();
    Ok(serde_json::to_string(&signed_block).unwrap())
  }

  #[napi]
  pub fn get_blocks(&self) -> napi::Result<Vec<String>> {
    Ok(
      self
        .micro
        .blocks
        .iter()
        .map(|block| serde_json::to_string(&block).unwrap())
        .collect(),
    )
  }
}
