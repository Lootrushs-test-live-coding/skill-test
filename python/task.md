# Python — 15 minutes

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

Implement `Ledger.release` in `skill-tests/python/ledger.py`.

## Rules

Check in this order. On any error, leave balances and escrows unchanged.

1. Unknown escrow → `NotFound`.
2. Caller is not the buyer or the arbiter → `NotAuthorized`.
3. Status is not `"funded"` → `BadStatus`.
4. Set status to `"released"`, set `amount` to `0`, and add the previous amount to the seller balance.

Seed used by `test_ledger.py`:

- Escrow `1`: buyer `10`, seller `20`, arbiter `30`, amount `500`, status `"funded"`. Seller balance starts at `25`.
- Escrow `2`: same parties, amount `100`, status `"refunded"`.

## Run

From `skill-tests/python/`:

```
python -m unittest test_ledger.py
```

## Done

`python -m unittest test_ledger.py` passes. The commit is on `develop`.

Push `develop` and open a pull request into `main`.

```
git push -u origin develop
```

The pull request base is `main` and the title is `Implement escrow release`.
