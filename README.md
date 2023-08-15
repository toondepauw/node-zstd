# @toondepauw/node-zstd

![CI workflow](https://github.com/toondepauw/node-zstd/actions/workflows/CI.yml/badge.svg)
[![latest-release](https://badgen.net/github/release/toondepauw/node-zstd)](https://github.com/toondepauw/node-zstd)
[![license](https://badgen.net/github/license/toondepauw/node-zstd)](https://github.com/toondepauw/node-zstd/blob/main/LICENCE.md)

Node.js addon for native [Zstandard](http://facebook.github.io/zstd/) encoding and decoding with support for dictionaries.

## Table of Contents

- [Installation](#installation)
- [Support matrix](#support-matrix)
- [API](#api)
- [Running Tests](#running-tests)
- [Releasing](#releasing)
- [License](#license)

## Installation

With npm:

```
npm install @toondepauw/node-zstd
```

With yarn:

```
yarn add @toondepauw/node-zstd
```

## Support matrix

|                  | node14 | node16 | node18 | node20 |
| ---------------- | ------ | ------ | ------ | ------ |
| Linux arm64 gnu  | ✓      | ✓      | ✓      | ✓      |
| Linux arm64 musl | ✓      | ✓      | ✓      | ✓      |
| Linux x64 gnu    | ✓      | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      | ✓      |
| macOS arm64      | ✓      | ✓      | ✓      | ✓      |
| macOS x64        | ✓      | ✓      | ✓      | ✓      |
| Windows x64      | ✓      | ✓      | ✓      | ✓      |
| FreeBSD x64      | ✓      | ✓      | ✓      | ✓      |

## API

### Decoder

```ts
export class Decoder {
  constructor(dictionary?: Buffer | undefined | null);
  decode(data: Buffer): Promise<Buffer>;
  decodeSync(data: Buffer): Buffer;
}
```

_These options can be provided as arguments to the decoder constructor._

| Option     | Type     | Requirement  | Description                  |
| ---------- | -------- | ------------ | ---------------------------- |
| dictionary | `Buffer` | **Optional** | Contents of dictionary file. |

### Encoder

```ts
export class Encoder {
  constructor(level: number, dictionary?: Buffer | undefined | null);
  encode(data: Buffer): Promise<Buffer>;
  encodeSync(data: Buffer): Buffer;
}
```

_These options can be provided as arguments to the encoder constructor._

| Option     | Type     | Requirement  | Description                                                                                                                 |
| ---------- | -------- | ------------ | --------------------------------------------------------------------------------------------------------------------------- |
| level      | `number` | **Required** | Zstd compression level. Can range between -7 (fastest) and 22 (slowest, best compression ratio). Level 3 is a good default. |
| dictionary | `Buffer` | **Optional** | Contents of dictionary file.                                                                                                |

## Usage

This package provides both an asynchronous and synchronous API. In most cases the asynchronous one is preferred as it is non-blocking. If blocking I/O is not a concern the synchronous API has a slight performance benefit.

### Async

```ts
import { Encoder, Decoder } from "@toondepauw/node-zstd";

const COMPRESSION_LEVEL = 3;

const encoder = new Encoder(COMPRESSION_LEVEL);
const decoder = new Decoder();

(async () => {
  const buff = Buffer.from("Hello, world!");
  const encoded = await encoder.encode(buff);
  const decoded = await decoder.decode(encoded);
})();
```

### Sync

```ts
import { Encoder, Decoder } from "@toondepauw/node-zstd";

const COMPRESSION_LEVEL = 3;

const encoder = new Encoder(COMPRESSION_LEVEL);
const decoder = new Decoder();

const buff = Buffer.from("Hello, world!");
const encoded = encoder.encodeSync(buff);
const decoded = decoder.decodeSync(encoded);
```

### Dictionary

A [dictionary](http://facebook.github.io/zstd/#small-data) provides a better compression ratio and improved compression speed for small files, but requires the dictionary to be present during both encoding and decoding.

```ts
import { readFileSync } from "fs";
import { Encoder, Decoder } from "@toondepauw/node-zstd";

const COMPRESSION_LEVEL = 3;
const dictionary = readFileSync("./dictionary");

// Same dictionary should be used for both encoding and decoding.
const encoder = new Encoder(COMPRESSION_LEVEL, dictionary);
const decoder = new Decoder(dictionary);
```

## Running Tests

`npm test`

## Releasing

CI will automatically publish when it detects a new release after:

```
npm version [major | minor | patch | ...]
git push --follow-tags origin main
```

## License

[MIT © 2023 - current Toon De Pauw](LICENCE.md)
