# Verification Trigger: Discovery + Vertical Slice 02

This branch exists only to trigger pull-request CI against the current `main` snapshot.

Expected gates:

- foundation/discovery validator;
- Rust workspace tests;
- Vertical Slice 01 black-box conformance;
- Vertical Slice 02 connection/path/backend-strategy conformance.

This marker does not add production behavior and should not be merged as product code.
