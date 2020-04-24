I've been on a Rust/Actix kick for a bit, and then grew some interest in systems communication via a personal toy project  

then into TCP as a form of communication, and found https://simplabs.com/blog/2018/06/11/actix/

## Actors

### The "actor model"

An actor

- can only be interacted with using `"messages"`

A `message` can be anything the actor can understand and can respond by:

- send a response
- send messages to other actors
- change its own state

```javascript
// a simplified example in JavaScript syntax:
class CounterActor {
  constructor() {
    this.count = 0;
  }

  onReceive(message) {
    if (message.type === 'plus-one') {
      this.count += 1;
    }

    return this.count;
  }
}
```

The `CounterActor` class 

- initialized with internal state called `count` that is set to zero and it responds to plus-one messages by increasing the `count` state and returning the new value.

The complexity of actors is relatively low, and that is because the complexity is usually hidden in the actor frameworks that are used to run these types of primitives in the end. One example of such an actor framework is `actix`, which we will have a closer look at now.

## actix

- low-level Rust actor framework that powers `actix-web`
  
To get started with actix, let's port our CounterActor above to Rust:

```rust
use actix::prelude::*;

// `PlusOne` message implementation
struct PlusOne;

impl Message for PlusOne {
    type Result = u32;
}

// `CounterActor` implementation
struct CounterActor {
    count: u32,
}

impl CounterActor {
    pub fn new() -> CounterActor {
        CounterActor { count: 0 }
    }
}

impl Actor for CounterActor {
    type Context = Context<Self>;
}

impl Handler<PlusOne> for CounterActor {
    type Result = u32;

    fn handle(&mut self, _msg: PlusOne, _ctx: &mut Context<Self>) -> u32 {
        self.count += 1;
        self.count
    }
}
```

- all structures need to be declared upfront, needed by Rust
- first import all the necessary things from the `actix::prelude` module, and then we define a `PlusOne` message
- In the JS impl the `message` had a `type` property, but there's a strict type system in Rust there is no need to explicitly declare that (it helps to actually compare the impls in each language)
- That leaves us with an empty `PlusOne` message, indicated by the struct `PlusOne` which does not have any content. The message does have a `Result` type though, defined by type `Result = u32;` which means "unsigned 32 bit integer".

The `CounterActor` impl is another struct which is roughly similar to a class in JS.

- It impls two traits `Actor` and `Handler`
- `Actor`defines that `CounterActor` as an actor that complies with the necessary interface to be run by the actix framework
- The `Context` type declaration can be used for more advanced impls, but we use default here
- then the `Handler` trait for the `PlusOne` message that we defined earlier => `impl Handler<PlusOne> for CounterActor`
  - in the `handle()` method we increment the count state and then return the new value to tell actix that this is our response to the message.

---

## Running our CounterActor

The following code will
1. start up system
2. startup our actor,
3. send a message,
4. wait for the response,
5. send another message,
6. wait for the response and finally exit the application:

```rust
// 1
let sys = actix::System::new("test");

// 2 
let counter: Addr<Syn, _> = Arbiter::start(|_| CounterActor::new());
let counter_addr_copy = counter.clone();

// 3
let result = counter.send(PlusOne)
    // 4
    .and_then(move |count| {
        println!("Count: {}", count);
        counter_addr_copy.send(PlusOne)
    })
    .map(|count| {
        println!("Count: {}", count);
        // 5
        Arbiter::system().do_send(actix::msgs::SystemExit(0));
    })
    .map_err(|error| {
        println!("An error occured: {}", error);
    });

// 6
Arbiter::handle().spawn(result);

sys.run();
```

- first set up a System to handle all actor interactions, `actix::System::new()` and pass it a name
- next, start an `Arbiter` in a new thread
  - `Syn` means that it is running in a separate thread, and that the `Arbiter` is the thing that controls that thread. 
  - `Arbiter::start()` call returns an `Addr` (short for address), that we can use to talk to the actor, via `counter`
   - `Addr` struct has methods like `send()` that can be used to send messages to the actor and receive their responses
   - Rust's borrow checker makes sure that data access can only happen in safe ways. Since we are (at least currently) not allowed to reuse it inside the callback. Instead we need to create a clone() and use that one instead.
- The `counter.send()` call returns a `Future`, which has several methods that can be used to assemble a pipeline of how to handle the result that the `Future` returns
- we use `.and_then()` to wait for the result of the `PlusOne` message, then print it out to the console, and then fire off another `PlusOne` message
- once that second message has returned we print the response again and then use a special system arbiter call to exit the process.

In Rust, a Future needs to be started explicitly. This difference exists for performance reasons, and is different than a JS `Promise` this way.  

To start the `Future` that we have assembled we use `Arbiter::handle().spawn()`, and then finally start the `System` once everything is wired up correctly to block the current thread until all actors have finished running
