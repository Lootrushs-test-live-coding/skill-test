# Skill tests — escrow release

Timed coding exercises (about 15 minutes each). Every skill implements the same idea: **release funds from an escrow** to the seller, with authorization and status checks.

Pick **one** skill folder. Do not edit the others.

## Skills

| Folder | Implement |
| --- | --- |
| [`react/`](react/) | `ReleasePanel` in `ReleasePanel.tsx` |
| [`node/`](node/) | `release` in `server.js` |
| [`python/`](python/) | `Ledger.release` in `ledger.py` |
| [`rust-backend/`](rust-backend/) | `Ledger::release` in `src/lib.rs` |
| [`go/`](go/) | `(*Ledger).Release` in `ledger.go` |
| [`java/`](java/) | `Ledger.release` in `Ledger.java` |
| [`csharp/`](csharp/) | `Ledger.Release` in `Ledger.cs` |
| [`ruby/`](ruby/) | `Ledger#release` in `ledger.rb` |
| [`cpp/`](cpp/) | `Ledger::release` in `ledger.cpp` |
| [`solidity/`](solidity/) | `release` in `src/EscrowRelease.sol` |
| [`solana/`](solana/) | `apply_release` in `programs/escrow_release/src/lib.rs` |

Each folder has a `task.md` with the full rules for that skill.

## Workflow

1. Create or switch to `develop` from the repository root:

   ```bash
   git switch -c develop
   # or, if it already exists:
   git switch develop
   ```

2. Implement only the skill you chose. Leave `main` unchanged.

3. Commit on `develop`:

   ```bash
   git add -A
   git commit -m "Implement escrow release"
   ```

4. Push and open a pull request into `main` with title **Implement escrow release**:

   ```bash
   git push -u origin develop
   ```
