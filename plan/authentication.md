# Authentication

Credentials management, login/logout, index authentication.

## Login/Logout/Token

- [x] authentication/credentials.md#login-text-store (from auth.rs::login_text_store)
- [x] authentication/credentials.md#login-password-stdin (from auth.rs::login_password_stdin)
- [x] authentication/credentials.md#login-token-stdin (from auth.rs::login_token_stdin)
- [x] authentication/credentials.md#token-text-store (from auth.rs::token_text_store)
- [x] authentication/credentials.md#logout-text-store (from auth.rs::logout_text_store)
- [x] authentication/credentials.md#auth-disabled-provider (from
      auth.rs::auth_disabled_provider_uses_text_store)
- [x] authentication/credentials.md#login-strips-simple-suffix (from
      auth.rs::login_text_store_strips_simple_suffix)
- [x] authentication/credentials.md#logout-strips-simple-suffix (from
      auth.rs::logout_text_store_strips_simple_suffix)
- [x] authentication/credentials.md#token-strips-simple-suffix (from
      auth.rs::token_text_store_strips_simple_suffix)
- [x] authentication/credentials.md#token-username (from auth.rs::token_text_store_username)
- [x] authentication/credentials.md#logout-multiple-usernames (from
      auth.rs::logout_text_store_multiple_usernames)

## Bazel Credential Helper

- [x] authentication/bazel-helper.md#basic-auth (from auth.rs::bazel_helper_basic_auth)
- [x] authentication/bazel-helper.md#token (from auth.rs::bazel_helper_token)
- [x] authentication/bazel-helper.md#no-credentials (from auth.rs::bazel_helper_no_credentials)
- [x] authentication/bazel-helper.md#invalid-json (from auth.rs::bazel_helper_invalid_json)
- [x] authentication/bazel-helper.md#invalid-uri (from auth.rs::bazel_helper_invalid_uri)
- [x] authentication/bazel-helper.md#username-in-uri (from auth.rs::bazel_helper_username_in_uri)
- [x] authentication/bazel-helper.md#unknown-username-in-uri (from
      auth.rs::bazel_helper_unknown_username_in_uri)

## Native Authentication

Keyring/system credential store integration - BLOCKED: requires native-auth feature, network access,
and system keyring.

### Package Installation

- [ ] BLOCKED: add-package-realm (from auth.rs::add_package_native_auth_realm) - requires
      native-auth feature + network
- [ ] BLOCKED: add-package (from auth.rs::add_package_native_auth) - requires native-auth feature +
      network

### Token Operations

- [ ] BLOCKED: token (from auth.rs::token_native_auth) - requires native-auth feature
- [ ] BLOCKED: token-realm (from auth.rs::token_native_auth_realm) - requires native-auth feature

### Login/Logout

- [ ] BLOCKED: login (from auth.rs::login_native_auth) - requires native-auth feature + network
- [ ] BLOCKED: login-token (from auth.rs::login_token_native_auth) - requires native-auth feature +
      network
- [ ] BLOCKED: logout (from auth.rs::logout_native_auth) - requires native-auth feature
- [ ] BLOCKED: logout-token (from auth.rs::logout_token_native_auth) - requires native-auth feature
- [ ] BLOCKED: login-url (from auth.rs::login_native_auth_url) - requires native-auth feature +
      network

### Credential Matching

- [ ] BLOCKED: prefix-match (from auth.rs::native_auth_prefix_match) - requires native-auth feature
- [ ] BLOCKED: host-fallback (from auth.rs::native_auth_host_fallback) - requires native-auth
      feature
