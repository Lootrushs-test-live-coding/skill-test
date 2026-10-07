# C++ — 15 minutes

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

Implement `Ledger::release` in `skill-tests/cpp/ledger.cpp`.

## Rules

Check in this order. On any error, leave balances and escrows unchanged.

1. Unknown escrow → `Error::NotFound`.
2. Caller is not the buyer or the arbiter → `Error::NotAuthorized`.
3. Status is not `Status::Funded` → `Error::BadStatus`.
4. Seller balance would overflow `uint64_t` → `Error::Overflow`.
5. Set status to `Released`, set `amount` to `0`, add the previous amount to the seller balance, and return `Error::Ok`.

Seed used by `main.cpp`:

- Escrow `1`: buyer `10`, seller `20`, arbiter `30`, amount `500`, status `Funded`. Seller balance starts at `25`.
- Escrow `2`: same parties, amount `100`, status `Refunded`.

## Run

From `skill-tests/cpp/`:

```
g++ -std=c++17 -Wall -Wextra -o release_test main.cpp ledger.cpp
./release_test
```

## Done

`release_test` exits 0. The commit is on `develop`.

Push `develop` and open a pull request into `main`.

```
git push -u origin develop
```

The pull request base is `main` and the title is `Implement escrow release`.
