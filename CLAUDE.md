YOU ARE MIGRATING THE TEST SUITE TO THE NEW MDTEST FRAMEWORK. REVIEW THE PLANS FOR THE MIGRATION. DO NOT STOP UNTIL YOU HAVE MIGRATED ALL TESTS THAT DO NOT REQUIRE NEW MDTEST FEATURES. DOCUMENT WHICH MDTEST FEATURES ARE MISSING. SOME TESTS MAY NOT BE MIGRATABLE, BUT MOST ARE.KEEP GOING UNTIL YOU ARE EXPLICITLY TOLD TO STOP. IF THE CONTEXT IS CLOSE TO FULL, THAT IS OKAY YOU CAN COMPACT AND KEEP GOING.



- Read CONTRIBUTING.md for guidelines on how to run tools
- ALWAYS attempt to add a test case for changed behavior
- PREFER integration tests, e.g., at `it/...` over unit tests
- When making changes for Windows from Unix, use `cargo xwin clippy` to check compilation
- NEVER perform builds with the release profile, unless asked or reproducing performance issues
- PREFER running specific tests over running the entire test suite
- AVOID using `panic!`, `unreachable!`, `.unwrap()`, unsafe code, and clippy rule ignores
- PREFER patterns like `if let` to handle fallibility
- ALWAYS write `SAFETY` comments following our usual style when writing `unsafe` code
- PREFER `#[expect()]` over `[allow()]` if clippy must be disabled
- PREFER let chains (`if let` combined with `&&`) over nested `if let` statements
