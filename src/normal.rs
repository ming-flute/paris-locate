#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::__private_exports_do_not_use::__export_format_stdout(format!($($arg)*), "\n")
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::__private_exports_do_not_use::__export_format_stdout(format!("<cyan><info></> <bright-yellow><on-cyan><underline>{}</>: {}", $crate::__private_exports_do_not_use::__export_function_name!(), format!($($arg)*)), "\n")
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::__private_exports_do_not_use::__export_format_stderr(format!("<red><cross></> <bright-yellow><on-red><underline>{}</>: {}", $crate::__private_exports_do_not_use::__export_function_name!(), format!($($arg)*)), "\n")
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::__private_exports_do_not_use::__export_format_stdout(format!("<yellow><warn></> <bright-yellow><on-white><underline>{}</>: {}", $crate::__private_exports_do_not_use::__export_function_name!(), format!($($arg)*)), "\n")
    }
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        $crate::__private_exports_do_not_use::__export_format_stdout(format!("<green><tick></> <bright-yellow><on-green><underline>{}</>: {}", $crate::__private_exports_do_not_use::__export_function_name!(), format!($($arg)*)), "\n")
    }
}
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::__private_exports_do_not_use::__export_format_stdout(format!("<magenta> <bright-yellow><on-bright-white><underline>Debug[{}]</>: {}", $crate::__private_exports_do_not_use::__export_function_name!(), format!($($arg)*)), "\n")
    }
}
