mod normal;
pub use paris::output::{format_stdout,format_stderr};
pub use stdext::function_name;



// Do not use directly, this is just so we can use `stdext` and `paris` even if they aren't directly defined as dependencies
// Export names are changed to make it harder to appear in editor autocomplete since `#[doc(hidden)]` doesn't seem to work.
#[doc(hidden)]
pub mod __private_exports_do_not_use {
    #[doc(hidden)]
    pub use stdext::function_name as __export_function_name;
    #[doc(hidden)]
    pub use paris::output::{format_stdout as __export_format_stdout,format_stderr as __export_format_stderr};
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macros() {
        log!("This <cyan>is <bright green>a log<//>!");
        info!(
            "<red>HAHAHAHAHA<///> <black><on green>{}</>",
            "the crate supports macros with colors!"
        );
        error!("This is going to <bright red>stderr</> {}", "WOOOO");
        warn!("This is a {} <yellow>BEWARE</>!", "warning");
        success!("{} went well, congrats!", "<bright green>Everything</>");

        match "a" {
            "a" => log!(
                "It works inside a match as well!!! {}",
                "<bright blue>finally</>"
            ),
            _ => unreachable!(),
        }
    }
}
