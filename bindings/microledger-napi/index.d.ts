/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export function intoIdentifier(pk: Buffer): string
export type JsMicroledger = Microledger
export class Microledger {
  constructor()
  preAnchorBlock(attachments: Array<string>, identifiers: Array<string>): string
  anchorBlock(block: string, identifier: string, signature: Buffer): string
  getBlocks(): Array<string>
}
