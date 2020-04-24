extern crate actix;

use actix::{
    actors::{Connect, Connector},
    prelude::*,
};

struct TcpClientActor;

impl Actor for TcpClientActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("TcpClientActor started: {:?}", ctx);

        Connector::from_registry()
            .send(Connect::host("towel.blinkenlights.nl:23"))
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
}

fn main() {
    let system = actix::System::new("tcp_test");

    let _tcp_client: Addr<Syn, _> = Arbiter::start(|_| TcpClientActor);

    system.run();
}
