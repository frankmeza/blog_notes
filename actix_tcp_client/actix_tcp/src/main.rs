extern crate actix;
extern crate tokio_codec;
extern crate tokio_io;

use actix::{
    actors::{Connect, Connector},
    prelude::*,
};
use tokio_codec::{FramedRead, LinesCodec};
use tokio_io::AsyncRead;

// CONSOLE LOGGER
// struct ConsoleLogger
// #[derive(Message)] struct ReceivedLine
// impl Actor for ConsoleLogger
// impl Handler<ReceivedLine> for ConsoleLogger

pub struct ConsoleLogger;

#[derive(Message)]
pub struct ReceivedLine {
    pub line: String,
}

impl Actor for ConsoleLogger {
    type Context = Context<Self>;
}

impl Handler<ReceivedLine> for ConsoleLogger {
    type Result = ();

    fn handle(&mut self, message: ReceivedLine, _ctx: &mut Context<Self>) {
        println!("{}", message.line);
    }
}

// TCP CLIENT
// struct TcpClientActor;
// impl Actor for TcpClientActor
// impl StreamHandler<String, std::io::Error> for TcpClientActor

pub struct TcpClientActor {
    recipient: Recipient<Syn, ReceivedLine>,
}

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
        if let Err(error) = self.recipient.do_send(ReceivedLine { line }) {
            println!("do_send failed: {}", error);
        }
    }
}

fn main() {
    let system = actix::System::new("tcp_test");

    let _logger: Addr<Syn, _> = Arbiter::start(|_| ConsoleLogger);

    let _tcp_client: Addr<Syn, _> = Arbiter::start(|_| TcpClientActor {
        recipient: _logger.recipient(),
    });

    system.run();
}
