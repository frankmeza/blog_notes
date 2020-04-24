# The Goal

- build an `Actor` that connects to a certain TCP server,
- listens for new messages, and
- forwards them to another recipient actor
  
## Project Setup

- `cargo new actix-tcp-example`
- `cargo run` => `"Hello world!"

## The actor

- add dependency `actix = "0.5"` to Cargo.toml file

```rust
// main.rs
extern crate actix;

use actix::prelude::*;

struct TcpClientActor;

impl Actor for TcpClientActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("TcpClientActor started: {:?}", ctx);
    }
}

fn main() {
    let system = actix::System::new("tcp_test");

    let _tcp_client: Addr<Syn, _> = Arbiter::start(|_| TcpClientActor);

    system.run();
}
```

- defines empty struct `TcpClientActor`

`impl Actor for TcpClientActor`
  - with type `Context`
  - add `started` hook to see printed message after start

in main.rs

- start a new Actix system
- create a new tcp client