# secret-msg

## Secret Message

Simple way to encrypt a message (No security whatsoever!!!)

This crate exposes 3 functions:

Encrypt and decrypt a messge:

`encode` and `decode`:

```rust
use secret_msg::{encode, decode};

let (secret, key) = encode("my_secret!");
assert_eq!(decode(&secret, key), "my_secret!");
let (secret, key) = encode(1234);
assert_eq!(decode(&secret, key), "1234");
```

Encrypt a message with no easy way to retrieve it back

`one_way_encode`:

```rust
use secret_msg::one_way_encode;

let sipher = one_way_encode("my_secret!");
assert_eq!(sipher, "1537");
let sipher = one_way_encode(158721);
assert_eq!(sipher, "2361");
```

License: MIT
