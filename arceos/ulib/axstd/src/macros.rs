//! Standard library macros

/// Prints to the standard output.
///
/// Equivalent to the [`println!`] macro except that a newline is not printed at
/// the end of the message.
///
/// [`println!`]: crate::println
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::__print_impl(format_args!($($arg)*));
    }
}

/// Prints to the standard output, with a newline.
#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {
        // $crate::io::__print_impl(format_args!("{}\n", format_args!($($arg)*)));
        // [INFO] EXERCISE 1
        // __print_impl
        //   -> arceos_api::stdio::ax_console_write_fmt
        //   -> axlog::print_fmt
        //   -> Logger.write_fmt(args)
        // Logger 是实现了 core::fmt::Write 这个 Trait 的零尺寸类型 (纯工具类，无需实例化)
        // 所有 println! 变为青色字体输出
        $crate::io::__print_impl(format_args!("\x1B[36m{}\x1B[0m\n", format_args!($($arg)*)));
    }
}