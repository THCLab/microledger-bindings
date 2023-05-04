# Microledger-js

Foreign Function Interface (FFI) layer to enable the use of [Microledger](https://github.com/THCLab/microledger) within a Node.js environment.

## Usage

In `bindings/microledger-js` run: 

```
npm i
npm run build
```

then you can run `example.js` with 
```
node example.js
```

## Interface overview

#### `intoIdentifier` 
Function that let you convert generated public key into nontransferable identifier. Currently works only for ed25519 keys.

#### `new` 
Microledger constructor.

#### `preAnchorBlock`
Creates matching block, that can be appended to microledger after signing. Takes list of data to be attached, and list of controlling identifiers.

Example: `micro.preAnchorBlock(["hello"], ["BFUJZUGPuI7WYheL5fPmHXcZ3G_U8GGnDM3hzpI46aC7"])`

#### `anchorBlock`
Checks provided block, verify signature and appends it to microledger. It takes block string, signer identifier and signature( as Buffer) as arguments. Currently works only for ed25519 signatures.

#### `getBlocks`
Returns all appended blocks as a string.
