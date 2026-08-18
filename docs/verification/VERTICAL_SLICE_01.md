# Vertical Slice 01 Verification Checklist

This file records the verifier gate for the first executable AgentFS slice.

Required evidence before the slice is called complete:

- [ ] `python scripts/validate_foundation.py`
- [ ] `cargo test --workspace --all-targets`
- [ ] `python -m pytest tests/conformance/test_vertical_slice_01.py -v`
- [ ] normalized home-relative path matches the protocol contract
- [ ] relative traversal above selected base fails closed
- [ ] absolute mode requires an absolute path
- [ ] risk mapping produces levels 0 through 4 correctly
- [ ] unknown operations fail closed

A checked box must correspond to fresh command output from CI or an equivalent environment. This document is not itself evidence that a check passed.
