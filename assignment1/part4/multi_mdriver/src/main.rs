use std::fs::File;

use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::{Arc, Mutex};
use std::thread;

const N: i64 = 1000;
const W: i64 = 54;
const INIT_VAL: i64 = 69;

fn main() {
    let file = Arc::new(Mutex::new(
        File::options()
            .read(true)
            .write(true)
            .open("/dev/mdriver")
            .unwrap(),
    ));
    let f = Arc::clone(&file);
    let mut f = f.lock().unwrap();
    f.seek(SeekFrom::Start(0)).unwrap();

    let val: i64 = INIT_VAL;
    let val_bytes = val.to_le_bytes();
    f.write(&val_bytes).unwrap();
    drop(f);
    let mut children = vec![];

    for _ in 0..W {
        let file_clone = Arc::clone(&file);

        children.push(thread::spawn(move || {
            for _ in 0..N {
                // println!("Thread {:?} is running", thread::current().id());
                let mut f = file_clone.lock().expect("Unable to lock file");
                f.seek(SeekFrom::Start(0)).unwrap();
                let mut buffer = [0; 8];
                let _bytes_written = f.read_exact(&mut buffer);
                let value: i64 = i64::from_le_bytes(buffer);
                let new_value = value + 1;
                let new_value_bytes = new_value.to_le_bytes();
                // println!("new value is {}", new_value);
                f.seek(SeekFrom::Start(0)).unwrap();
                f.write(&new_value_bytes).unwrap();
                drop(f);
            }
        }));
    }

    for child in children {
        let _ = child.join();
    }

    let mut file = File::options()
        .read(true)
        .write(true)
        .open("/dev/mdriver")
        .unwrap();

    let _ = file.seek(SeekFrom::Start(0));

    let mut buffer = [0; 8];

    let _bytes_written = file.read_exact(&mut buffer);
    let value: i64 = i64::from_le_bytes(buffer);
    println!("Target {}", N * W + INIT_VAL);
    println!("Actual {}", value);
}
