pub mod utils {
    use rand::Rng;
    pub fn gen_nums(c: u32) -> Vec<u16> {
        let mut v: Vec<u16> = Vec::new();

        for _index in 0..c {
            let n = rand::thread_rng().gen_range(1..10);
            v.push(n)
        }

        v
    }

    use std::io;
    pub fn wait_for_input() -> String {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input
    }
}
