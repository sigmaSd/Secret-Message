# secret-msg

## Secret Message

Simple way to encrypt a message (No security whatsoever!!!)

This crate exposes **SecretMessage** trait:

encrypt and decrypt a messge:

**encrypt** and **decrypt**:

```rust
use secret_msg::SecretMessage;

let (secret, key) = "my_secret!".encrypt();
assert_eq!(secret.decrypt(key), "my_secret!");
let (secret, key) = 1234.encrypt();
assert_eq!(secret.decrypt(key), "1234");
```

or use a custom key:

```rust
use secret_msg::SecretMessage;
let secret = "cool secret".encrypt_with_key(58794);
assert_eq!(secret.decrypt(58794), "cool secret");
```
encrypt a message with no easy way to retrieve it back:

**one_way_encrypt**:

```rust
use secret_msg::SecretMessage;

let cipher = "my_secret!".one_way_encrypt();
assert_eq!(cipher, "1537");
let cipher = 158721.one_way_encrypt();
assert_eq!(cipher, "2361");
```

License: MIT
