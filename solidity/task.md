# Solidity smart contract — 15 minutes

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

Implement `release` in `skill-tests/solidity/src/EscrowRelease.sol`.

The escrow is funded in the constructor. Leave the constructor, storage, errors, and the event as they are.

## Rules

1. If `msg.sender` is not `buyer` or `arbiter`, revert `NotAuthorized`.
2. If `status` is not `Funded`, revert `BadStatus`.
3. Set `status` to `Released` and `amount` to `0` before any external call.
4. Send the previous `amount` to `seller` with `call`. If `call` returns false, revert `TransferFailed`.
5. Emit `Released` with the seller and the amount paid.

## Run

From `skill-tests/solidity/`:

```
forge test
```

## Done

`forge test` passes. The commit is on `develop`.

Push `develop` and open a pull request into `main`.

```
git push -u origin develop
```

The pull request base is `main` and the title is `Implement escrow release`.
