pub(crate) mod helpers {
  use base64::{engine::general_purpose, Engine};
  use ed25519_dalek::{PublicKey, Signature as EdLibSignature, Verifier as EdLibVerifier};
  use serde::{Deserialize, Serialize};

  use microledger::{verifier::Verifier, Identifier, Result, Signature};

  #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
  pub struct EasyIdentifier(pub String);

  impl Identifier for EasyIdentifier {}

  #[derive(Serialize, Deserialize, Clone)]
  pub struct EdSignature(pub String);
  impl Signature for EdSignature {
    type Identifier = EasyIdentifier;

    fn get_signer(&self) -> Option<Self::Identifier> {
      Some(EasyIdentifier("Identifier1".to_string()))
    }
  }
  pub struct EdVerifier(pub PublicKey);

  impl Verifier for EdVerifier {
    type Signature = EdSignature;

    fn verify(&self, data: &[u8], s: Vec<Self::Signature>) -> Result<bool> {
      Ok(s.iter().all(|sig| {
        let raw_sig = general_purpose::STANDARD.decode(&sig.0).unwrap();
        self
          .0
          .verify(data, &EdLibSignature::from_bytes(&raw_sig).unwrap())
          .is_ok()
      }))
    }
  }
}
