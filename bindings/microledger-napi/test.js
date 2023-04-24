const {Microledger} = require('./index')

const nacl = require("tweetnacl");
const util = require("tweetnacl-util");
const assert = require('assert');
const {Base64} = require('js-base64')

const currentKeyPair = nacl.sign.keyPair();
const nextKeyPair = nacl.sign.keyPair();

let keyB64 = Base64.encode(currentKeyPair.publicKey, false) // true to remove padding =
console.log(keyB64)
let nextKeyB64 = Base64.encode(nextKeyPair.publicKey, false)
// let curPrefix = "D".concat(keyB64) // attach derivation code.
// let nextPrefix = "D".concat(nextKeyB64)

let mic = new Microledger(keyB64)
let block = mic.preAnchorBlock(["hello"], [keyB64])
console.log("initial block: \n" + block + "\n")
let signature = nacl.sign.detached(Buffer.from(block, 'utf8'), currentKeyPair.secretKey);

let signed_block = mic.anchorBlock(block, Buffer.from(signature));
console.log("signed initial block: \n" + signed_block + "\n")

console.log("Microledger blocks: \n" + mic.getBlocks() + "\n")
assert.equal(mic.getBlocks().length, 1)
console.log("------------- Add second block --------------------\n")

let second_block = mic.preAnchorBlock(["hello there"], [nextKeyB64])
console.log("next block: \n" + second_block + "\n")
let second_signature = nacl.sign.detached(Buffer.from(second_block, 'utf8'), currentKeyPair.secretKey);

let signed_second_block = mic.anchorBlock(second_block, Buffer.from(second_signature));
console.log("signed next block: \n" + signed_second_block + "\n")

console.log("Microledger blocks: \n" + mic.getBlocks() + "\n")
assert.equal(mic.getBlocks().length, 2)

// This will panic because of wrong signature
// console.log("-------------- Try to add block with wrong signature -------------------\n")
// const wrongKeyPair = nacl.sign.keyPair();
// let next_block = mic.preAnchorBlock(["is it correct?"], [nextKeyB64])
// console.log("next block: \n" + next_block + "\n")
// let wrong_signature = nacl.sign.detached(Buffer.from(second_block, 'utf8'), wrongKeyPair.secretKey);

// let wrong_signature_block = mic.anchorBlock(next_block, Buffer.from(wrong_signature));
// console.log("wrongly signed next block: \n" + wrong_signature_block + "\n")
// console.log("Microledger blocks: \n" + mic.getBlocks() + "\n")