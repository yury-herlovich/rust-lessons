pub mod utils {
    use std::io;
    pub fn wait_for_input() -> String {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input
    }
}
