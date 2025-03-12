use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;

pub fn runtime() -> &'static Runtime {
    static INSTANCE: OnceCell<Runtime> = OnceCell::new();

    INSTANCE.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .thread_name("greptime_ingester")
            .enable_all()
            .build()
            .expect("Failed to create tokio runtime")
    })
}
