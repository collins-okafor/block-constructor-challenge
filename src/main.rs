// use std::fs::File;
// use std::io::{self, BufRead};
// use std::path::Path;
// use std::collections::HashMap;

// // Define a structure to represent transactions
// #[derive(Debug, Clone)]
// struct Transaction {
//     txid: String,
//     fee: u32,
//     weight: u32,
//     parent_txids: Vec<String>,
// }

// impl Transaction {
//     fn new(txid: String, fee: u32, weight: u32, parent_txids: Vec<String>) -> Self {
//         Transaction {
//             txid,
//             fee,
//             weight,
//             parent_txids,
//         }
//     }
// }

// fn main() {
//     // Read mempool data from CSV file
//     if let Ok(lines) = read_lines("./mempool.csv") {
//         // Initialize a vector to store transaction data
//         let mut transactions: Vec<Transaction> = vec![];

//         // Iterate over lines in the CSV file
//         for line in lines {
//             if let Ok(row) = line {
//                 // Split the row by commas and extract transaction data
//                 let cols: Vec<&str> = row.split(',').collect();
//                 let txid = cols[0].to_string();
//                 let fee: u32 = cols[1].parse().unwrap();
//                 let weight: u32 = cols[2].parse().unwrap();
//                 let parent_txids: Vec<String> = if cols.len() > 3 {
//                     cols[3].split(';').map(|s| s.to_string()).collect()
//                 } else {
//                     Vec::new()
//                 };

//                 // Create a new Transaction object and add it to the transactions vector
//                 let transaction = Transaction::new(txid, fee, weight, parent_txids);
//                 transactions.push(transaction);
//             }
//         }

//         // Initialize variables for block construction
//         let max_weight = 4000000;
//         let mut block_weight = 0;
//         let mut selected_txids: HashMap<String, bool> = HashMap::new(); // Use HashMap for efficiency

//         // Sort transactions by fee in descending order
//         let mut cloned_transactions = transactions.clone();
//         cloned_transactions.sort_by_key(|tx| -(tx.fee as i32));

//         // Construct the block by selecting transactions with maximum fee
//         for transaction in cloned_transactions {
//             // Check if adding the transaction exceeds block weight limit or if it conflicts with parents
//             if block_weight + transaction.weight <= max_weight
//                 && transaction
//                     .parent_txids
//                     .iter()
//                     .all(|parent_txid| selected_txids.contains_key(parent_txid))
//             {
//                 // Add the transaction to the block
//                 block_weight += transaction.weight;
//                 selected_txids.insert(transaction.txid.clone(), true);
//             }
//         }

//         // Output selected transactions in the correct order
//         if selected_txids.is_empty() {
//             println!("No valid block could be constructed.");
//         } else {
//             for txid in selected_txids.keys() {
//                 println!("{}", txid);
//             }
//         }
//     } else {
//         eprintln!("Error reading mempool.csv: {}", io::Error::last_os_error());
//     }
// }

// // Function to read lines from a file
// // fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::

// // Function to read lines from a file
// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where
//     P: AsRef<Path>,
// {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

// Define a structure to represent transactions
#[derive(Debug, Clone)]
struct Transaction {
    txid: String,
    fee: u32,
    weight: u32,
    parent_txids: Vec<String>,
}

impl Transaction {
    fn new(txid: String, fee: u32, weight: u32, parent_txids: Vec<String>) -> Self {
        Transaction {
            txid,
            fee,
            weight,
            parent_txids,
        }
    }
}

fn main() {
    // Read mempool data from CSV file
    if let Ok(lines) = read_lines("./mempool.csv") {
        // Initialize a vector to store transaction data
        let mut transactions: Vec<Transaction> = vec![];

        // Iterate over lines in the CSV file
        for line in lines {
            if let Ok(row) = line {
                // Split the row by commas and extract transaction data
                let cols: Vec<&str> = row.split(',').collect();
                let txid = cols[0].to_string();
                let fee: u32 = cols[1].parse().unwrap();
                let weight: u32 = cols[2].parse().unwrap();
                let parent_txids: Vec<String> = if cols.len() > 3 {
                    cols[3].split(';').map(|s| s.to_string()).collect()
                } else {
                    Vec::new()
                };

                // Create a new Transaction object and add it to the transactions vector
                let transaction = Transaction::new(txid, fee, weight, parent_txids.clone());
                println!("Read transaction: {:?}", &transaction);
                transactions.push(transaction);
            }
        }

        // Initialize variables for block construction
        let max_weight = 4000000;
        let mut block_weight = 0;
        let mut selected_txids: HashSet<String> = HashSet::new();

        // Sort transactions by fee in descending order
        let mut cloned_transactions = transactions.clone();
        cloned_transactions.sort_by_key(|tx| -(tx.fee as i32));

        // Construct the block by selecting transactions with maximum fee
        for transaction in cloned_transactions {
            // Check if adding the transaction exceeds block weight limit or if it conflicts with parents
            if block_weight + transaction.weight <= max_weight
                && transaction
                    .parent_txids
                    .iter()
                    .all(|parent_txid| selected_txids.contains(parent_txid))
            {
                // Add the transaction to the block
                block_weight += transaction.weight;
                selected_txids.insert(transaction.txid.clone());
                println!("Added transaction to block: {}", transaction.txid);
            }
        }

        // Output selected transactions in the correct order
        for txid in &selected_txids {
            println!("{}", txid);
        }
    }
}

// Function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
