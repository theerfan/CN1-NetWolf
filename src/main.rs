pub const STATIC_DIR: &'static str = "./static/";
pub const BUF_SIZE: usize = 8192;
mod node;
mod dir;
mod udp;
mod tcp;
use futures::executor::block_on;
use std::sync::Mutex;

async fn async_main() {
    let mut _nodes = node::read_starting_nodes();
    let mutex = Mutex::new(&mut _nodes);
    udp::udp_server(mutex).await;
}

fn main() -> std::io::Result<()> {
    block_on(async_main());
    Ok(())
}

