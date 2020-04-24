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