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

encrypt a message with no easy way to retrieve it back

**one_way_encrypt**:

```rust
use secret_msg::SecretMessage;

let sipher = "my_secret!".one_way_encrypt();
assert_eq!(sipher, "1537");
let sipher = 158721.one_way_encrypt();
assert_eq!(sipher, "2361");
```

License: MIT
