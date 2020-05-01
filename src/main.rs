use futures::executor::block_on;

mod engine;
mod house;

fn main() {
    block_on(house::main("House"));
}
