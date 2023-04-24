use base64::{engine::general_purpose, Engine};
use ed25519_dalek::PublicKey;
use helper::helpers::{EasyIdentifier, EdSignature, EdVerifier};
use microledger::{
  block::Block,
  microledger::MicroLedger,
  seal_bundle::{SealBundle, SealData},
  Encode,
};
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
mod helper;
use std::sync::Arc;

#[napi(js_name = "Microledger")]
struct JsMicroledger {
  micro: MicroLedger<EdSignature, EdVerifier, EasyIdentifier>,
}

#[napi]
impl JsMicroledger {
  #[napi(constructor)]
  pub fn new(pk: String) -> Self {
    let pk_bytes = general_purpose::STANDARD.decode(pk).expect("Wrong base64");
    let pk = PublicKey::from_bytes(&pk_bytes).unwrap();
    let validator = Arc::new(EdVerifier(pk));
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

    let mut controlling_ids: Vec<EasyIdentifier> = vec![];
    for _i in identifiers {
      let id = EasyIdentifier("Identifier1".to_string());

      controlling_ids.push(id);
    }
    let block = self
      .micro
      .pre_anchor_block(controlling_ids, &seal_bundle)
      .unwrap();
    let block_str = String::from_utf8(block.encode().unwrap()).unwrap();

    Ok(block_str)
  }

  #[napi]
  pub fn anchor_block(&mut self, block: String, signature: Buffer) -> napi::Result<String> {
    let block: Block<EasyIdentifier> = serde_json::from_str(&block).unwrap();

    let signature = EdSignature(general_purpose::STANDARD.encode(&signature));
    let signed_block = block.to_signed_block(vec![signature]);

    self.micro.anchor(signed_block.clone()).unwrap();
    Ok(serde_json::to_string(&signed_block).unwrap())
  }

  #[napi]
  pub fn get_blocks(&self) -> napi::Result<Vec<String>> {
    Ok(self.micro.blocks.iter().map(|block| serde_json::to_string(&block).unwrap()).collect())
  }
}
