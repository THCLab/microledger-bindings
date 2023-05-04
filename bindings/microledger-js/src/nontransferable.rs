use keri::{
  event_message::signature::Nontransferable,
  prefix::{BasicPrefix, SelfSigningPrefix},
};
use microledger::{verifier::Verifier, Identifier, Result, Signature};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct NontransferableIdentifier(pub BasicPrefix);

#[derive(Serialize, Clone)]
pub struct NontransferableSignature(pub Nontransferable);

impl Identifier for NontransferableIdentifier {}

impl Signature for NontransferableSignature {
  type Identifier = NontransferableIdentifier;

  fn get_signer(&self) -> Option<Self::Identifier> {
    match &self.0 {
      Nontransferable::Indexed(_) => todo!(),
      Nontransferable::Couplet(couplets) => Some(NontransferableIdentifier(couplets[0].0.clone())),
    }
  }
}

impl NontransferableSignature {
  pub fn new(signer_id: String, signature: Vec<u8>) -> Self {
    let signer: BasicPrefix = signer_id.parse().expect("Can't parse signer id");
    let signature = SelfSigningPrefix::Ed25519Sha512(signature);
    NontransferableSignature(Nontransferable::Couplet(vec![(signer, signature)]))
  }
}

pub struct NontransferableVerifier;

impl Verifier for NontransferableVerifier {
  type Signature = NontransferableSignature;

  fn verify(&self, data: &[u8], s: Vec<Self::Signature>) -> Result<bool> {
    Ok(s.into_iter().all(|sig| {
      match sig.0 {
        Nontransferable::Indexed(_) => todo!(),
        Nontransferable::Couplet(couplets) => couplets
          .into_iter()
          .all(|(pk, sig)| pk.verify(data, &sig).expect("Error while verifing")),
      }
    }))
  }
}
