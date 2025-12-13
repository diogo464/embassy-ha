mod std_async_tcp;
use embassy_executor::Executor;
use static_cell::StaticCell;
pub use std_async_tcp::AsyncTcp;

pub static EXECUTOR: StaticCell<Executor> = StaticCell::new();

#[macro_export]
macro_rules! example_main {
    () => {
        fn main() {
            // Initialize tracing if tracing feature is enabled
            #[cfg(feature = "tracing")]
            {
                tracing_subscriber::fmt()
                    .with_max_level(tracing::Level::DEBUG)
                    .init();
            }

            let executor = common::EXECUTOR.init(Executor::new());
            executor.run(|spawner| {
                spawner.spawn(main_task(spawner).unwrap());
            });
        }
    };
}
