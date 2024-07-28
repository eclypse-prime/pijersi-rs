use std::io;

use pijersi_rs::ugi::UgiEngine;

/// Runs the UGI protocol engine
fn main() -> ! {
    rayon::ThreadPoolBuilder::new().num_threads(8).build_global().unwrap();

    let mut ugi_engine = UgiEngine::new();
    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");
        command.truncate(command.trim_end().len());
        ugi_engine.get_command(&command);
    }
}
