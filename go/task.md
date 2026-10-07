# Go — 15 minutes

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

Implement `(*Ledger).Release` in `skill-tests/go/ledger.go`.

## Rules

Check in this order. On any error, leave balances and escrows unchanged.

1. Unknown escrow → `ErrNotFound`.
2. Caller is not the buyer or the arbiter → `ErrNotAuthorized`.
3. Status is not `Funded` → `ErrBadStatus`.
4. Seller balance would overflow `int64` → `ErrOverflow`.
5. Set status to `Released`, set `amount` to `0`, and add the previous amount to the seller balance.

Seed used by `ledger_test.go`:

- Escrow `1`: buyer `10`, seller `20`, arbiter `30`, amount `500`, status `Funded`. Seller balance starts at `25`.
- Escrow `2`: same parties, amount `100`, status `Refunded`.

## Run

From `skill-tests/go/`:

```
go test
```

## Done

`go test` passes. The commit is on `develop`.

Push `develop` and open a pull request into `main`.

```
git push -u origin develop
```

The pull request base is `main` and the title is `Implement escrow release`.
