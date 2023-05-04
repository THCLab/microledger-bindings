use std::sync::Arc;

use cesrox::{group::Group, payload::Payload, ParsedData};
use keri::event_message::signature::Nontransferable;
use microledger::{
  block::{Block, SignedBlock},
  error::Error,
  microledger::MicroLedger,
  Encode,
};

use crate::nontransferable::{
  NontransferableIdentifier, NontransferableSignature, NontransferableVerifier,
};

/// Wrapper that adds CESR block representation for signed block.
struct NontransferableBlock(pub SignedBlock<NontransferableIdentifier, NontransferableSignature>);

impl TryFrom<ParsedData> for NontransferableBlock {
  type Error = Error;

  fn try_from(parsed: ParsedData) -> std::result::Result<Self, Self::Error> {
    let block: Block<NontransferableIdentifier> = match parsed.payload {
      Payload::JSON(json) => serde_json::from_slice(&json).map_err(Error::EncodeError)?,
      Payload::CBOR(_) => todo!(),
      Payload::MGPK(_) => todo!(),
    };
    let signatures: Vec<_> = parsed
      .attachments
      .into_iter()
      .map(|g| match g {
        Group::NontransReceiptCouples(couplets) => {
          NontransferableSignature(Nontransferable::Couplet(
            couplets
              .into_iter()
              .map(|(bp, sp)| (bp.into(), sp.into()))
              .collect(),
          ))
        }
        _ => todo!(),
      })
      .collect();
    Ok(NontransferableBlock(block.to_signed_block(signatures)))
  }
}

impl NontransferableBlock {
  pub fn to_cesr(&self) -> Result<Vec<u8>, Error> {
    let payload = Payload::JSON(Encode::encode(&self.0.block)?);
    let groups = self
      .0
      .signatures
      .iter()
      .map(|nt| match &nt.0 {
        Nontransferable::Indexed(_) => todo!(),
        Nontransferable::Couplet(pairs) => Group::NontransReceiptCouples(
          pairs
            .iter()
            .map(|(bp, sp)| (bp.clone().into(), sp.clone().into()))
            .collect(),
        ),
      })
      .collect();

    let d = ParsedData {
      payload,
      attachments: groups,
    };
    d.to_cesr().map_err(|_e| Error::CesrError)
  }
}

pub fn to_cesr_str(
  signed_block: &SignedBlock<NontransferableIdentifier, NontransferableSignature>,
) -> Result<String, Error> {
  String::from_utf8(NontransferableBlock(signed_block.clone()).to_cesr()?)
    .map_err(|_e| Error::CesrError)
}

pub fn parse_microledger(
  stream: &[u8],
) -> Result<
  MicroLedger<NontransferableSignature, NontransferableVerifier, NontransferableIdentifier>,
  Error,
> {
  let (_rest, parsed_stream) = cesrox::parse_many(stream).map_err(|_e| Error::CesrError)?;
  let mut microledger = MicroLedger::new(Arc::new(NontransferableVerifier));
  parsed_stream.into_iter().try_for_each(|pd| {
    let block: NontransferableBlock = pd.try_into()?;
    microledger.anchor(block.0)
  })?;
  Ok(microledger)
}
