# The Goal

- build an `Actor` that connects to a certain TCP server,
- listens for new messages, and
- forwards them to another recipient actor
  
## Project Setup

- `cargo new actix-tcp-example`
- `cargo run` => `"Hello world!"

## The Actor

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

## Connect to a TCP Server

`started` now looks like this:

```rust
fn started(&mut self, ctx: &mut Self::Context) {
    println!("TcpClientActor started: {:?}", ctx);

    Connector::from_registry()
        .send(Connect::host("127.0.0.1:9000"))
        .into_actor(self)
        .map(|res, _act, ctx| match res {
            Ok(stream) => {
                println!("very success: {:?}", &stream);
            }
            Err(err) => {
                println!("very virus: {}", err);
                ctx.stop();
            }
        })
        .map_err(|err, _act, ctx| {
            println!("very virus 2: {}", err);
            ctx.stop();
        })
        .wait(ctx);
}
```

new thing here is `Connector::from_registry()`

- actix comes with a small number of built-in actors and one of them is the so-called `Connector`. 
- The `Connector` actor can be used to perform DNS lookups, or connect to remote servers via TCP, which is exactly what we want!
- `Connector::from_registry()` returns an `Addr` instance for the `Connector`
- we can use the `send()` method on it to send a `Connect` message and wait for the answer (using a `Future`).
- the `into_actor()` modifier transforms the `Future` into another kind of `Future`, 
  - where we get passed not only the result,
  - but also the actor instance and the context in any callbacks that we implement
  - This is helpful to work around the Rust compiler complaining about lifetime issues

Now we're ready to use the result and we do so by implementing a callback for the `map()` method. We'll keep it simple for now and just print a message to the terminal whether the connection was successful or not.  

Since we have to handle two different kinds of errors here (`ConnectorError` and `MailboxError`) we have to implement two different error handlers too.  

They have the same code but since the error classes are different, they can't easily share the same implementation unfortunately  

Finally, we block the current thread using the `wait()` method until the `Future` is resolved.

Great! We have successfully opened up a TCP connection to the `towel.blinkenlights.nl` server  

