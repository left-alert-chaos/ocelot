//!# go
//!This module holds code to parse a `go` UCI command.

use crate::physical::*;
use crate::bot::uci::safe_parse_action;

pub struct Go {
    cutoff: SearchCutoff,
    searchmoves: Vec<Box<dyn Action>>,
    wtime: Option<f64>,
    btime: Option<f64>,
    winc: Option<f64>,
    binc: Option<f64>,
    movestogo: Option<i32>,
}

impl Go {
    pub fn parse(command: String, board: &mut Board) -> Result<Self, ()> {
        let mut words = command.split_whitespace();
        
        //find cutoff
        let mut cutoff = SearchCutoff::Infinite;
        let mut unit: &str = "";
        while let Some(word) = words.next() {
            match word {
                "infinite" => {
                    cutoff = SearchCutoff::Infinite;
                }
                "depth" | "nodes" | "mate" => unit = word,
                _ => {
                    if let Ok(num) = word.to_string().parse::<i32>() {
                        match unit {
                            "depth" => cutoff = SearchCutoff::Depth(num),
                            "nodes" => cutoff = SearchCutoff::Nodes(num),
                            "mate" => cutoff = SearchCutoff::Mate(num),
                            _ => continue,
                        }

                        break;
                    }
                },
            }
        }

        //find searchmoves
        let mut searchmoves = Vec::new();
        let keyword = words.next();
        if keyword == Some("searchmoves") {
            for word in words {
                if let Ok(action) = safe_parse_action(word.to_string(), board) {
                    searchmoves.push(action);
                }
            }
        } else if keyword.is_none() {
            return Ok(Self {
                cutoff,
                searchmoves,
                wtime: None,
                btime: None,
                winc: None,
                binc: None,
                movestogo: None,
            })
        }

        Err(())
    }
}

#[derive(Copy, Clone)]
pub enum SearchCutoff {
    Infinite, //only time limit applies
    Mate(i32),
    Nodes(i32),
    Depth(i32),
}

