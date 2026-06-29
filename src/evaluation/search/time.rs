//!# time
//!This module holds code to test machine performance and change allowed time based on that.

use crate::evaluation::SearchTree;
use crate::physical::Board;
use std::time::{Duration, Instant};

///Store information about performance and use that to generate allowed search times
struct SearchDiagnostic {
    seconds: u64,
}

impl SearchDiagnostic {
    ///Run a diagnostic and produce Self
    pub fn run() -> Self {
        let before = Instant::now();
        
        //create a default board and search it
        let board: Board = Default::default();
        let _ = SearchTree::new(&board, 5, f64::INFINITY).safe_best_move();
        let s = before.elapsed().as_secs();
        println!("{s}");

        Self {
            seconds: before.elapsed().as_secs()
        }
    }

    pub fn allowed_time_for_node(&self, absolute_depth: u64) -> Duration {
        //manual power calculation equal to 40^x
        let mut nodes_at_depth = 40;
        for _ in 1..absolute_depth {
            nodes_at_depth *= 40;
        }

        //evenly distribute between all sibling nodes
        let seconds = self.seconds as f64 / nodes_at_depth as f64;
        eprintln!("SearchDiagnostic::allowed_time_for_node(): {seconds} seconds allowed for node at depth {absolute_depth}");
        Duration::new(0, 0)
    }
}
