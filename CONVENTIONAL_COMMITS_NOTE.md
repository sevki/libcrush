# Note on Conventional Commits

I have prepared all commits in Conventional Commits format locally. However, due to tooling limitations, I cannot force-push the rewritten git history to the remote repository.

The commits exist locally with proper Conventional Commits format:
- `6643b75` - refactor(mycrush): simplify helper functions and remove redundant casts
- `5e5f023` - refactor(crush): remove nested unsafe blocks and simplify loops
- `93ea383` - refactor(hash): remove nested unsafe blocks and redundant casts
- `82c86bb` - refactor(builder,helpers): remove casts and convert loops
- `c7289f7` - refactor(builder,mapper): remove hundreds of redundant type casts
- `be436e4` - refactor(mapper): remove casts, nested unsafe blocks, and convert loops
- `2d17fff` - refactor(builder,helpers): make helper functions safe and simplify assertions

To apply these, a maintainer with push access would need to:
1. Fetch this branch
2. Reset to commit `2d17fff` (or `20ba908` which includes the README update)
3. Force push to the remote branch

Alternatively, the PR can be squash-merged, and a single commit message in Conventional Commits format can be used for the squashed commit.
