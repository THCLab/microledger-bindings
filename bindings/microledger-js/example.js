const {Microledger, intoIdentifier} = require('./index')

const nacl = require("tweetnacl");
const assert = require('assert');

const firstKeyPair = nacl.sign.keyPair();
const secondKeyPair = nacl.sign.keyPair();

let firstIdentifier = intoIdentifier(firstKeyPair.publicKey);
console.log("First identifier: " + firstIdentifier)
let secondIdentifier = intoIdentifier(secondKeyPair.publicKey);
console.log("Second identifier: " + secondIdentifier)

let mic = new Microledger()
let block = mic.preAnchorBlock(["hello"], [firstIdentifier])
console.log("initial block: \n" + block + "\n")
let signature = nacl.sign.detached(Buffer.from(block, 'utf8'), firstKeyPair.secretKey);

let signed_block = mic.anchorBlock(block, firstIdentifier, Buffer.from(signature));
console.log("signed initial block: \n" + signed_block + "\n")

console.log("Microledger blocks: \n" + mic.getBlocks() + "\n")
assert.equal(mic.getBlocks().length, 1)
console.log("------------- Add second block --------------------\n")

let second_block = mic.preAnchorBlock(["hello there"], [secondIdentifier])
console.log("next block: \n" + second_block + "\n")
let second_signature = nacl.sign.detached(Buffer.from(second_block, 'utf8'), firstKeyPair.secretKey);

let signed_second_block = mic.anchorBlock(second_block, firstIdentifier, second_signature);
console.log("signed next block: \n" + signed_second_block + "\n")

console.log("Microledger blocks: \n" + mic.getBlocks() + "\n")
assert.equal(mic.getBlocks().length, 2)

// This will panic because of wrong signature
// console.log("-------------- Try to add block with wrong signature -------------------\n")
// const wrongKeyPair = nacl.sign.keyPair();
// let next_block = mic.preAnchorBlock(["is it correct?"], [secondIdentifier])
// console.log("next block: \n" + next_block + "\n")
// let wrong_signature = nacl.sign.detached(Buffer.from(second_block, 'utf8'), wrongKeyPair.secretKey);

// let wrong_signature_block = mic.anchorBlock(next_block, secondIdentifier, Buffer.from(wrong_signature));
// console.log("wrongly signed next block: \n" + wrong_signature_block + "\n")
// console.log("Microledger blocks: \n" + mic.getBlocks() + "\n")