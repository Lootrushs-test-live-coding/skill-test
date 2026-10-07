# Solana smart contract — 15 minutes

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

Implement `apply_release` in `skill-tests/solana/programs/escrow_release/src/lib.rs`.

The `release` instruction and the lamport transfer are already written. They call `apply_release`.

## Rules

`apply_release` updates the escrow and returns how many lamports to pay the seller.

1. If `authority` is not `escrow.buyer` or `escrow.arbiter`, return `NotAuthorized`.
2. If `escrow.status` is not `Funded`, return `BadStatus`.
3. If `seller` is not `escrow.seller`, return `SellerMismatch`.
4. On any error, leave the escrow unchanged.
5. Remember `amount`, set status to `Released`, set `amount` to `0`, and return the remembered amount.

## Run

From `skill-tests/solana/`:

```
cargo test
```

## Done

`cargo test` passes. The commit is on `develop`.

Push `develop` and open a pull request into `main`.

```
git push -u origin develop
```

The pull request base is `main` and the title is `Implement escrow release`.
