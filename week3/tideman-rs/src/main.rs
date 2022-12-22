use std::{
    env,
    io::{self, Write},
};

// Max number of candidates
const MAX: usize = 9;
const MAX_PAIRS: usize = MAX * (MAX - 1) / 2;

type Preferences = [[i32; 9]; 9];
type Locked = [[bool; 9]; 9];

// Each pair has a winner, loser
struct Pair {
    winner: usize,
    loser: usize,
}

fn get_int(message: &str) -> i32 {
    let string_input = get_string(message);
    string_input.parse::<i32>().expect("Invalid number")
}

fn get_string(message: &str) -> String {
    let mut input = String::new();
    print!("{}", message);
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().to_string();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for invalid usage
    if args.len() < 2 {
        println!("Usage: tideman [candidate ...]");
        return;
    }

    // Populate array of candidates
    let candidate_count = args.len() - 1;
    if candidate_count > MAX {
        println!("Maximum number of candidates is {}", MAX);
        return;
    }

    let candidates = &args[1..];
    let voter_count = get_int("Number of voters: ");
    let mut preferences: Preferences = [[0; MAX as usize]; MAX as usize];

    // Query for votes
    for _i in 0..voter_count {
        // ranks[i] is voter's ith preference
        let mut ranks = vec![0; candidate_count];

        // Query for each rank
        for j in 0..candidate_count {
            let message = format!("Rank {}: ", j + 1);
            let name: String = get_string(&message);

            if !vote(j, name, &mut ranks, candidates) {
                println!("Invalid vote.");
                return;
            }
        }

        record_preferences(ranks, &mut preferences);

        println!();
    }

    let mut pairs: Vec<Pair> = Vec::with_capacity(MAX_PAIRS);
    let mut locked: Locked = [[false; MAX as usize]; MAX as usize];

    add_pairs(preferences, &mut pairs);

    sort_pairs(&mut pairs, preferences);
    for pair in &pairs {
        println!(
            "winner: {} loser: {}, vote_count {}",
            pair.winner, pair.loser, preferences[pair.winner][pair.loser]
        );
    }
    lock_pairs(&mut locked, pairs);
    print_winner(locked, candidate_count, candidates);
}

// Update ranks given a new vote
fn vote(rank: usize, name: String, ranks: &mut [usize], candidates: &[String]) -> bool {
    let candidate_index_search = candidates.iter().position(|r| r.as_str() == name);

    match candidate_index_search {
        Some(candidate_index) => {
            ranks[rank] = candidate_index;
            true
        }
        None => false,
    }
}

// Update preferences given one voter's ranks
fn record_preferences(ranks: Vec<usize>, preferences: &mut Preferences) {
    let last_index = ranks.len() - 1;
    for (candidate_index, candidate) in ranks.iter().enumerate() {
        if candidate_index == last_index {
            break;
        }
        let winner = candidate;
        let loser = ranks[candidate_index + 1];
        preferences[*winner][loser] += 1;
    }
}

// Record pairs of candidates where one is preferred over the other
fn add_pairs(preferences: Preferences, pairs: &mut Vec<Pair>) {
    for (winner_index, preference) in preferences.iter().enumerate() {
        for (loser_index, vote_count) in preference.iter().enumerate() {
            if *vote_count == 0 {
                continue;
            }
            let pair = Pair {
                winner: winner_index,
                loser: loser_index,
            };
            pairs.push(pair);
        }
    }
}

// Sort pairs in decreasing order by strength of victory
fn sort_pairs(pairs: &mut [Pair], preferences: Preferences) {
    pairs.sort_by(|a, b| preferences[b.winner][b.loser].cmp(&preferences[a.winner][a.loser]))
}

fn is_cycle(origin_candidate: usize, candidate_to_lock: usize, graph: Locked) -> bool {
    if graph[candidate_to_lock][origin_candidate] {
        return true;
    }

    for vertex_index in 0..MAX {
        if graph[vertex_index][origin_candidate] {
            return is_cycle(vertex_index, candidate_to_lock, graph);
        }
    }

    false
}

// Lock pairs into the candidate graph in order, without creating cycles
fn lock_pairs(lock_graph: &mut Locked, pairs: Vec<Pair>) {
    for pair in pairs {
        if is_cycle(pair.winner, pair.loser, *lock_graph) {
            continue;
        }
        lock_graph[pair.winner][pair.loser] = true;
    }
}

// Print the winner of the election
fn print_winner(locked: Locked, candidate_count: usize, candidates: &[String]) {
    let mut winner_index = 0;

    for i in 0..candidate_count {
        let mut has_edge = false;
        for j in 0..candidate_count {
            if locked[j][i] {
                has_edge = true;
            }
        }
        if !has_edge {
            winner_index = i;
            break;
        }
    }
    println!("The winner is {}", candidates[winner_index])
}
