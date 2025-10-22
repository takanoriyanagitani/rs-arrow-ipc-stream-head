use rs_arrow_ipc_stream_head::stdin2head2stdout;
use std::process::exit;

fn main() {
    let length = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(10);

    if let Err(e) = stdin2head2stdout(length) {
        eprintln!("Error: {}", e);
        exit(1);
    }
}
