use rust_template::utils::sum;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let sum = sum(1, 2);
    info!("The sum is {sum}");
}
