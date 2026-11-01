# lint-adoption

Problem: chess should run the shared lints, as the real test of whether they are any good.

L1 Engine behaviour does not change. Perft and search results are unchanged.
L2 No allow-lists or lint configuration.
L3 Tests go through public APIs.
D1 Install the lint runner in CI from Studio-Terza/meta-dev at a pinned commit.
D2 Fix violations in small PRs into this branch, by crate.
D3 Remove meta-xtask once the ported lints pass. after: D2
Out: new engine features, strength changes, the elegance lint.
