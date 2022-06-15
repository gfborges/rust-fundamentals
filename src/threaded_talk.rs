use std::thread;
use std::sync::mpsc;
use std::time::Duration;

pub fn start() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move|| {
        let msgs = vec![
            "Hey, let me in",
            "Could you please let me in?",
            "Oh no, i guess i will go home then",
        ];

        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for msg in rx {
        println!("🕵️  {}", msg);
        println!("👮 Go away, i won't let you in");
    }
    println!("...\n👮 Finally, that guy went home");

    handle.join().expect("Falied to close thread correctly");
}