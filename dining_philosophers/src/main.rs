extern crate rand;

use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn thinking(&self) {
        let thinking_time = rand::thread_rng().gen_range(1, 150);
        thread::sleep(Duration::from_millis(thinking_time));
        println!("~~~~~~ {} go to dining ~~~~~~", self.name);
    }

    fn eat(&self, table: &Table) {
        println!("{0} pick fork No. {1}", self.name, self.left);
        let _left = table.forks[self.left].lock().unwrap();
        println!("----- Current forks: {:?}", table.forks);

        println!("{0} pick fork No. {1}", self.name, self.right);
        let _right = table.forks[self.right].lock().unwrap();
        println!("----- Current forks: {:?}", table.forks);

        println!("{} is eating......", self.name);
        thread::sleep(Duration::from_millis(1000));
        println!("!!!!! {} is done eating !!!!!", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });

    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gillers Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4),
    ];

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            let table = table.clone();

            thread::spawn(move || {
                p.thinking();
                p.eat(&table);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
