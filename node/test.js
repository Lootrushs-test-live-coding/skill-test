"use strict";

const { test } = require("node:test");
const assert = require("node:assert/strict");
const { seed, release, createServer } = require("./server");

test("buyer releases", () => {
  const store = seed();
  const result = release(store, 1, 10);
  assert.equal(result.status, 200);
  assert.deepEqual(result.body, {
    id: 1,
    status: "released",
    seller: 20,
    amount: 500,
  });
  assert.equal(store.balances.get(20), 525);
  assert.equal(store.escrows.get(1).status, "released");
  assert.equal(store.escrows.get(1).amount, 0);
  assert.equal(store.escrows.get(2).status, "refunded");
});

test("arbiter releases", () => {
  const store = seed();
  const result = release(store, 1, 30);
  assert.equal(result.status, 200);
  assert.equal(store.balances.get(20), 525);
});

test("seller is rejected", () => {
  const store = seed();
  const result = release(store, 1, 20);
  assert.equal(result.status, 403);
  assert.deepEqual(result.body, { error: "not_authorized" });
  assert.equal(store.balances.get(20), 25);
  assert.equal(store.escrows.get(1).status, "funded");
  assert.equal(store.escrows.get(1).amount, 500);
});

test("second release is rejected", () => {
  const store = seed();
  assert.equal(release(store, 1, 10).status, 200);
  const again = release(store, 1, 10);
  assert.equal(again.status, 409);
  assert.deepEqual(again.body, { error: "bad_status" });
  assert.equal(store.balances.get(20), 525);
});

test("missing escrow", () => {
  const store = seed();
  const result = release(store, 99, 10);
  assert.equal(result.status, 404);
  assert.deepEqual(result.body, { error: "not_found" });
  assert.equal(store.balances.get(20), 25);
});

test("refunded escrow is rejected", () => {
  const store = seed();
  const result = release(store, 2, 10);
  assert.equal(result.status, 409);
  assert.equal(store.balances.get(20), 25);
  assert.equal(store.escrows.get(2).amount, 100);
});

test("missing user is rejected", () => {
  const store = seed();
  const result = release(store, 1, null);
  assert.equal(result.status, 400);
  assert.deepEqual(result.body, { error: "bad_user" });
  assert.equal(store.escrows.get(1).status, "funded");
});

test("http header and route", async () => {
  const store = seed();
  const server = createServer(store);
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  const address = server.address();
  if (address === null || typeof address === "string") {
    throw new Error("expected a tcp port");
  }
  const { port } = address;
  try {
    const missing = await fetch(`http://127.0.0.1:${port}/escrows/1/release`, { method: "POST" });
    assert.equal(missing.status, 400);
    assert.equal(store.escrows.get(1).status, "funded");

    const ok = await fetch(`http://127.0.0.1:${port}/escrows/1/release`, {
      method: "POST",
      headers: { "x-user-id": "10" },
    });
    assert.equal(ok.status, 200);
    assert.deepEqual(await ok.json(), {
      id: 1,
      status: "released",
      seller: 20,
      amount: 500,
    });
    assert.equal(store.balances.get(20), 525);
  } finally {
    await new Promise((resolve, reject) => server.close((err) => (err ? reject(err) : resolve())));
  }
});
