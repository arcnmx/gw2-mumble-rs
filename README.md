# gw2_mumble
MumbleLink bindings specific to Guild Wars 2.

```toml
gw2_mumble = { git = "https://github.com/zerthox/gw2-mumble-rs" }
```

```rs
use gw2_mumble::MumbleLink;

let link = MumbleLink::new().unwrap();
let camera = link.read_camera();
let player_pos = link.read_avatar();
```

[Serde](https://serde.rs) support can be enabled with the `"serde"` feature.

Parsing of the player identity JSON is supported when enabling the `"json"` feature:

```toml
gw2_mumble = { git = "https://github.com/zerthox/gw2-mumble-rs", features = ["json"] }
```

```rs
let identity = link.parse_identity();
```
  