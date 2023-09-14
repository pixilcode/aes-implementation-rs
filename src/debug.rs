#[macro_export]
macro_rules! debug {
	($( $print_values:tt ),* ) => {
		if std::env::var("DEBUG").is_ok() {
			println!($( $print_values ),*);
		}
	};
}