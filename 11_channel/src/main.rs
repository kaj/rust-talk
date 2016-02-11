// These declarations allow us to use these standard library
// definitions without writing out their full module path.
use std::fs::Metadata;
use std::io::Result;
use std::path::PathBuf;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;
use std::thread::spawn;

fn worker_loop(files:   Receiver<PathBuf>,
               results: Sender<(PathBuf, Result<Metadata>)>) {
    for path_buf in files {
        let metadata = std::fs::metadata(&path_buf);
        results.send((path_buf, metadata)).unwrap();
    }
}

fn main() {
    let paths = vec!["/home/kaj/.zshrc",
                     "/home/kaj/.emacs",
                     "/home/kaj/nonesuch",
                     "/home/kaj/.cargo",
                     "/home/kaj/.golly"];
    
    let worker;

    // Create a channel the worker thread can use to send
    // results to the main thread.
    let (worker_tx, main_rx) = channel();

    {
        // Create a channel the main thread can use to send
        // filenames to the worker.
        let (main_tx, worker_rx) = channel();

        // Start the worker thread.
        worker = spawn(move || {
            worker_loop(worker_rx, worker_tx);
        });

        // Send paths to the worker thread to check.
        for path in paths {
            main_tx.send(PathBuf::from(path)).unwrap();
        }

        // main_tx is dropped here, which closes the channel.
        // The worker will exit after it has received everything
        // we sent.
    }

    // We could do other work here, while waiting for the
    // results to come back.
    for (path, result) in main_rx {
        match result {
            Ok(metadata) =>
                println!("Size of {:?}: {}", &path, metadata.len()),
            Err(err) =>
                println!("Error for {:?}: {}", &path, err)
        }
    }

    worker.join().unwrap();
}

