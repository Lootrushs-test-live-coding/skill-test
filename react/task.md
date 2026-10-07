# React — 15 minutes

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

Implement `ReleasePanel` in `skill-tests/react/ReleasePanel.tsx`.

## Rules

Props:

- `seller` and `amount` are text to show.
- `status` is `"funded"`, `"released"`, or `"refunded"`.
- `role` is `"buyer"`, `"seller"`, or `"arbiter"`.
- `onRelease` returns a promise.

Render:

- `Seller: {seller}`
- `Amount: {amount}`
- A button named `Release`.

The button is enabled only when all of these are true:

- `status` is `"funded"`
- `role` is `"buyer"` or `"arbiter"`
- a release is not already in flight
- this panel has not already released successfully

Behavior:

1. While `onRelease` is in flight, show `Releasing…` and disable the button.
2. When `onRelease` resolves, show `Released` and keep the button disabled.
3. When the initial `status` is `"released"`, show `Released` and keep the button disabled.
4. When `onRelease` rejects with an `Error`, show `error.message` and enable the button again if the role and status still allow it.
5. When the rejection is not an `Error`, show `Release failed`.

## Done

Refresh the running app and confirm the five behaviors. The commit is on `develop`.

Push `develop` and open a pull request into `main`.

```
git push -u origin develop
```

The pull request base is `main` and the title is `Implement escrow release`.
