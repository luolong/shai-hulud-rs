#[macro_export]
macro_rules! print_status {
    (ORANGE, $message:expr) => {
        println!("{}", console::style($message).bold().yellow());
    };
    (GREEN, $message:expr) => {
        println!("{}", console::style($message).bold().green());
    };
    (BLUE, $message:expr) => {
        println!("{}", console::style($message).bold().blue());
    };
}
