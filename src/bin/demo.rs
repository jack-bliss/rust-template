use rust_template::utils::sum;

#[doc(alias = "demo")]
/// Demo alternative binary. Has access to the lib.
/// ```bash
/// cargo make demo;
/// # "This is a test script! 3"
/// ```
fn main() {
    println!("This is a test script! {}", sum(1, 2));
}
