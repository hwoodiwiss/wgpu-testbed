use futures::executor;
fn main() {
    executor::block_on(wgpu_testbed_lib::run());
}
