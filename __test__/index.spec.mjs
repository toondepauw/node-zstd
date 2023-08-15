import test from "ava";
import { readFileSync } from "fs";
import { Decoder, Encoder } from "../index.js";

const syncMacro = test.macro({
  exec(t, { level, dict }, input) {
    const encoder = new Encoder(level, dict);
    const decoder = new Decoder(dict);
    const encoded = encoder.encodeSync(input);
    const decoded = decoder.decodeSync(encoded);
    t.deepEqual(decoded, input);
  },
  title(_provided, { level, dict }) {
    return `sync: level ${level} ${dict ? "with dictionary" : ""}`;
  },
});

const asyncMacro = test.macro({
  async exec(t, { level, dict }, input) {
    const encoder = new Encoder(level, dict);
    const decoder = new Decoder(dict);
    const encoded = await encoder.encode(input);
    const decoded = await decoder.decode(encoded);
    t.deepEqual(decoded, input);
  },
  title(_provided, { level, dict }) {
    return `async: level ${level} ${dict ? "with dictionary" : ""}`;
  },
});

const input = readFileSync("__test__/test.json");
const dict = readFileSync("__test__/dictionary");

// Synchronous encode / decode tests
test(syncMacro, { level: 1 }, input);
test(syncMacro, { level: 10 }, input);
test(syncMacro, { level: 22 }, input);
test(syncMacro, { level: 1, dict }, input);
test(syncMacro, { level: 10, dict }, input);
test(syncMacro, { level: 22, dict }, input);

// Asynchronous encode / decode tests
test(asyncMacro, { level: 1 }, input);
test(asyncMacro, { level: 10 }, input);
test(asyncMacro, { level: 22 }, input);
test(asyncMacro, { level: 1, dict }, input);
test(asyncMacro, { level: 10, dict }, input);
test(asyncMacro, { level: 22, dict }, input);

test("dictionary mismatch", async (t) => {
  // encoder uses dictionary but decoder doesn't
  const encoder = new Encoder(1, dict);
  const decoder = new Decoder();
  const encoded = encoder.encodeSync(input);

  const syncErr = t.throws(() => decoder.decodeSync(encoded));
  const asyncErr = await t.throwsAsync(decoder.decode(encoded));
  t.is(syncErr.message, "Dictionary mismatch");
  t.is(asyncErr.message, "Dictionary mismatch");
});

test("invalid arg Encoder", (t) => {
  const levelErr = { code: "NumberExpected" };
  t.throws(() => new Encoder(), levelErr);
  t.throws(() => new Encoder("invalid"), levelErr);

  const dictErr = { code: "InvalidArg" };
  t.throws(() => new Encoder(1, "invalid"), dictErr);
  t.throws(() => new Encoder(1, true), dictErr);
});

test("invalid arg Decoder", (t) => {
  const dictErr = { code: "InvalidArg" };
  t.throws(() => new Decoder("invalid"), dictErr);
  t.throws(() => new Decoder(true), dictErr);
});
