extern crate beanstalker;

use beanstalker::Beanstalker;

fn main() {
    let mut bs = Beanstalker::connect("127.0.0.1:11300").unwrap();
    let _ = bs.put(0, 0, 60, "{\"Name\":\"Firs data\"}");
}
