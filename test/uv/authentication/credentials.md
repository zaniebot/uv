# Authentication - Credentials Management

Tests for `uv auth login`, `uv auth logout`, and `uv auth token` commands with text-based credential
storage.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Login text store

<!-- Derived from [`auth::login_text_store`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/auth.rs#L1076-L1204) -->

Basic login/logout functionality with username/password and token authentication.

Login with username and password:

```console
$ uv auth login https://pypi-proxy.fly.dev/basic-auth/simple --username public --password heron
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Stored credentials for public@https://pypi-proxy.fly.dev/basic-auth
```

Login with token:

```console
$ uv auth login https://example.com/simple --token test-token
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Stored credentials for https://example.com/
```

Empty username fails:

```console
$ uv auth login https://example.com/simple --username "" --password testpass
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Username cannot be empty
```

Empty password fails:

```console
$ uv auth login https://example.com/simple --username testuser --password ""
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Password cannot be empty
```

HTTP fails for non-local hosts:

```console
$ uv auth login http://example.com/simple --username testuser --password testpass
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: invalid value 'http://example.com/simple' for '<SERVICE>': HTTPS is required for non-local hosts

For more information, try '--help'.
```

Other protocols fail:

```console
$ uv auth login ftp://example.com/simple --username testuser --password testpass
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: invalid value 'ftp://example.com/simple' for '<SERVICE>': Unsupported scheme: ftp

For more information, try '--help'.
```

HTTP is allowed on localhost (127.0.0.1):

```console
$ uv auth login http://127.0.0.1/simple --username testuser --password testpass
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Stored credentials for testuser@http://127.0.0.1/
```

HTTP is allowed on localhost:

```console
$ uv auth login http://localhost/simple --username testuser --password testpass
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Stored credentials for testuser@http://localhost/
```

## Login password stdin

<!-- Derived from [`auth::login_password_stdin`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/auth.rs#L1208-L1247) -->

Passwords can be read from stdin using `-` as the password value.

Create a password file:

```text
# file: password.txt
secret-password
```

Login with password from stdin:

```console
$ uv auth login https://example.com/simple --username testuser --password - < password.txt
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Stored credentials for testuser@https://example.com/
```

## Login token stdin

<!-- Derived from [`auth::login_token_stdin`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/auth.rs#L1249-L1286) -->

Tokens can be read from stdin using `-` as the token value.

Create a token file:

```text
# file: token.txt
my-secret-token-12345
```

Login with token from stdin:

```console
$ uv auth login https://example.com/simple --token - < token.txt
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Stored credentials for https://example.com/
```

## Token text store

<!-- Derived from [`auth::token_text_store`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/auth.rs#L1288-L1351) -->

The `uv auth token` command retrieves stored credentials.

Login with credentials first:

```console
$ uv auth login https://example.com/simple --username testuser --password testpass
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Stored credentials for testuser@https://example.com/
```

Retrieve the password:

```console
$ uv auth token https://example.com/simple --username testuser
success: true
exit_code: 0
----- stdout -----
testpass

----- stderr -----
```

## Logout text store

<!-- Derived from [`auth::logout_text_store`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/auth.rs#L1353-L1415) -->

The `uv auth logout` command removes stored credentials.

Login first:

```console
$ uv auth login https://test.example.com/simple --username testuser --password testpass
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Stored credentials for testuser@https://test.example.com/
```

Logout:

```console
$ uv auth logout https://test.example.com/simple --username testuser
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Removed credentials for testuser@https://test.example.com/
```

Verify credentials are gone:

```console
$ uv auth token https://test.example.com/simple --username testuser
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to fetch credentials for testuser@https://test.example.com/simple
```

## Auth disabled provider

<!-- Derived from [`auth::auth_disabled_provider_uses_text_store`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/auth.rs#L1417-L1454) -->

When keyring provider is set to `disabled`, credentials are stored in text format.

Login with disabled provider:

```console
$ UV_KEYRING_PROVIDER=disabled uv auth login https://example.com/simple --username testuser --password testpass
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Stored credentials for testuser@https://example.com/
```

Retrieve token with disabled provider:

```console
$ UV_KEYRING_PROVIDER=disabled uv auth token https://example.com/simple --username testuser
success: true
exit_code: 0
----- stdout -----
testpass

----- stderr -----
```

## Login strips simple suffix

<!-- Derived from [`auth::login_text_store_strips_simple_suffix`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/auth.rs#L1456-L1539) -->

The `/simple` and `/+simple` suffixes are stripped when storing credentials.

Login with `/simple` suffix:

```console
$ uv auth login https://example.com/simple --username testuser --password testpass
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Stored credentials for testuser@https://example.com/
```

Login with `/+simple` suffix:

```console
$ uv auth login https://devpi.example.com/root/+simple --username devpiuser --password devpipass
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Stored credentials for devpiuser@https://devpi.example.com/root
```

## Logout strips simple suffix

<!-- Derived from [`auth::logout_text_store_strips_simple_suffix`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/auth.rs#L1542-L1594) -->

The `/simple` and `/+simple` suffixes are stripped for logout operations.

Login with `/simple` suffix:

```console
$ uv auth login https://example.com/simple --username testuser --password testpass
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Stored credentials for testuser@https://example.com/
```

Logout using same URL with `/simple`:

```console
$ uv auth logout https://example.com/simple --username testuser
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Removed credentials for testuser@https://example.com/
```

Login with `/+simple` suffix:

```console
$ uv auth login https://devpi.example.com/root/+simple --username devpiuser --password devpipass
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Stored credentials for devpiuser@https://devpi.example.com/root
```

Logout using URL with `/+simple`:

```console
$ uv auth logout https://devpi.example.com/root/+simple --username devpiuser
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Removed credentials for devpiuser@https://devpi.example.com/root
```

## Token strips simple suffix

<!-- Derived from [`auth::token_text_store_strips_simple_suffix`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/auth.rs#L1597-L1645) -->

The `/simple` suffix is stripped when retrieving tokens.

Login with `/simple` suffix:

```console
$ uv auth login https://example.com/simple --username testuser --password testpass
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Stored credentials for testuser@https://example.com/
```

Retrieve token using URL with `/simple`:

```console
$ uv auth token https://example.com/simple --username testuser
success: true
exit_code: 0
----- stdout -----
testpass

----- stderr -----
```

Login with token and `/simple` suffix:

```console
$ uv auth login https://token.example.com/simple --token secret-token
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Stored credentials for https://token.example.com/
```

Retrieve token using URL with `/simple`:

```console
$ uv auth token https://token.example.com/simple
success: true
exit_code: 0
----- stdout -----
secret-token

----- stderr -----
```

## Token username

<!-- Derived from [`auth::token_text_store_username`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/auth.rs#L1648-L1745) -->

Token retrieval requires matching usernames when credentials are stored with a username.

Login with specific username:

```console
$ uv auth login https://example.com/simple --username testuser --password testpass
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Stored credentials for testuser@https://example.com/
```

Retrieve token with matching username:

```console
$ uv auth token https://example.com/simple --username testuser
success: true
exit_code: 0
----- stdout -----
testpass

----- stderr -----
```

Retrieve token without username fails:

```console
$ uv auth token https://example.com/simple
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to fetch credentials for https://example.com/simple
```

Retrieve token with different username fails:

```console
$ uv auth token https://example.com/simple --username otheruser
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to fetch credentials for otheruser@https://example.com/simple
```

## Logout multiple usernames

<!-- Derived from [`auth::logout_text_store_multiple_usernames`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/auth.rs#L1748-L1824) -->

Multiple usernames can be stored for the same service and logged out independently.

Login with two different usernames:

```console
$ uv auth login https://example.com/simple --username user1 --password pass1
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Stored credentials for user1@https://example.com/
```

```console
$ uv auth login https://example.com/simple --username user2 --password pass2
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Stored credentials for user2@https://example.com/
```

Logout one specific username:

```console
$ uv auth logout https://example.com/simple --username user1
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Removed credentials for user1@https://example.com/
```

Verify first user is gone:

```console
$ uv auth token https://example.com/simple --username user1
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to fetch credentials for user1@https://example.com/simple
```

Verify second user remains:

```console
$ uv auth token https://example.com/simple --username user2
success: true
exit_code: 0
----- stdout -----
pass2

----- stderr -----
```

Logout without username (defaults to `__token__`) fails when only named users exist:

```console
$ uv auth logout https://example.com/simple
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No matching entry found for https://example.com/
```
