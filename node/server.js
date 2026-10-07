"use strict";

const http = require("http");

function seed() {
  return {
    escrows: new Map([
      [1, { buyer: 10, seller: 20, arbiter: 30, amount: 500, status: "funded" }],
      [2, { buyer: 10, seller: 20, arbiter: 30, amount: 100, status: "refunded" }],
    ]),
    balances: new Map([[20, 25]]),
  };
}

function release(_store, _escrowId, _userId) {
  // Implement this function.
  throw new Error("implement release");
}

function send(res, status, body) {
  const payload = JSON.stringify(body);
  res.writeHead(status, {
    "content-type": "application/json",
    "content-length": Buffer.byteLength(payload),
  });
  res.end(payload);
}

function createServer(store) {
  return http.createServer((req, res) => {
    const match = /^\/escrows\/(\d+)\/release$/.exec(req.url || "");
    if (req.method !== "POST" || !match) {
      send(res, 404, { error: "not_found" });
      return;
    }
    const header = req.headers["x-user-id"];
    const userId = typeof header === "string" && /^\d+$/.test(header) ? Number(header) : null;
    try {
      const result = release(store, Number(match[1]), userId);
      send(res, result.status, result.body);
    } catch (err) {
      send(res, 500, { error: err instanceof Error ? err.message : "error" });
    }
  });
}

if (require.main === module) {
  const port = Number(process.env.PORT || 3000);
  createServer(seed()).listen(port, () => {
    console.log(`listening on ${port}`);
  });
}

module.exports = { seed, release, createServer };
