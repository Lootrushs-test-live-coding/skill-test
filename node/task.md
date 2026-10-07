# Node — 15 minutes

## Branch

Work in this whole project on `develop`. From the repository root:

```
git switch -c develop
```

If `develop` already exists:

```
git switch develop
```

Edit this skill only. When you are done, commit on `develop`:

```
git add -A
git commit -m "Implement escrow release"
```

Leave `main` unchanged.

## Task

Implement `release` in `skill-tests/node/server.js`.

The HTTP server is already written. It accepts `POST /escrows/:id/release` and reads `x-user-id`. A missing or non-numeric header is passed as `null`.

## Rules

Return `{ status, body }`. Check in this order. On any error, leave the store unchanged.

1. `userId === null` → `400` `{ error: "bad_user" }`.
2. Unknown escrow → `404` `{ error: "not_found" }`.
3. Caller is not the buyer or the arbiter → `403` `{ error: "not_authorized" }`.
4. Status is not `"funded"` → `409` `{ error: "bad_status" }`.
5. Set status to `"released"`, set `amount` to `0`, add the previous amount to the seller balance, and return `200`:

```js
{ id, status: "released", seller, amount }
```

`amount` in the body is the amount paid, not `0`.

Seed:

- Escrow `1`: buyer `10`, seller `20`, arbiter `30`, amount `500`, status `"funded"`. Seller balance starts at `25`.
- Escrow `2`: same parties, amount `100`, status `"refunded"`.

## Run

From `skill-tests/node/`:

```
npm test
```

## Done

`npm test` passes. The commit is on `develop`.

Push `develop` and open a pull request into `main`.

```
git push -u origin develop
```

The pull request base is `main` and the title is `Implement escrow release`.
