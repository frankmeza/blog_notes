extern crate actix;
extern crate tokio_codec;
extern crate tokio_io;

use actix::{
    actors::{Connect, Connector},
    prelude::*,
};
use tokio_codec::{FramedRead, LinesCodec};
use tokio_io::AsyncRead;

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

                    let (r, w) = stream.split();
                    let line_reader = FramedRead::new(r, LinesCodec::new());
                    ctx.add_stream(line_reader);
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

impl StreamHandler<String, std::io::Error> for TcpClientActor {
    fn handle(&mut self, line: String, _ctx: &mut Self::Context) {
        println!("{}", line);
    }
}

fn main() {
    let system = actix::System::new("tcp_test");

    let _tcp_client: Addr<Syn, _> = Arbiter::start(|_| TcpClientActor);

    system.run();
}
