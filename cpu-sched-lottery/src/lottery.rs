// No need for globals

use std::{env, process};

use rand::{rngs::StdRng, Rng, SeedableRng};

struct Node {
    tickets: u64,
    next: Option<Box<Self>>,
}

fn insert(tickets: u64, head: Option<Box<Node>>, gtickets: &mut u64) -> Option<Box<Node>> {
    *gtickets += tickets;
    Some(Box::new(Node {
        tickets,
        next: head,
    }))
}

fn print_list(head: &Option<Box<Node>>) {
    let mut curr = head;
    println!("List: ");
    while let Some(node) = curr {
        println!("{}", node.tickets);
        curr = &node.next;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: lottery <seed> <loops>");
        process::exit(1);
    }

    let seed: u64 = args[1].parse::<u64>().unwrap();
    let loops: u64 = args[2].parse::<u64>().unwrap();
    let mut rng = StdRng::seed_from_u64(seed);
    let mut head: Option<Box<Node>> = None;
    let mut gtickets: u64 = 0;

    // populate list with some number of jobs, each
    // with some number of tickets
    head = insert(50, head, &mut gtickets);
    head = insert(100, head, &mut gtickets);
    head = insert(25, head, &mut gtickets);

    print_list(&head);

    for _ in 0..loops {
        let mut counter: u64 = 0;
        let winner = rng.gen_range(0..gtickets); // get winner

        let mut curr = &head;
        while let Some(node) = curr {
            counter += node.tickets;
            if counter > winner {
                break; // found the winner
            }
            curr = &node.next;
        }
        // current is the winner: schedule it
        // print_list(&head);
        println!("winner: {} {}", winner, curr.as_ref().unwrap().tickets);
    }
}
