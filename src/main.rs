use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Transaction {
    txid: String,
    fee: u64,
    weight: u32,
    parents: Vec<String>
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut transactions: HashMap<String, Transaction> = HashMap::new();
    let max_block_weight = 4_000_000;

    // Read transactions from mempool.csv
    let file: File = File::open("./mempool.csv.xlsx")?;
    let reader: BufReader<File> = BufReader::new(file);
    for line in reader.lines() {
        let line: String = line?;
        let mut parts: std::str::Split<'_, char> = line.split(',');
        let txid: String = parts.next().unwrap().to_string();
        let fee: u64 = parts.next().unwrap().parse::<u64>()?;
        let weight: u32 = parts.next().unwrap().parse::<u32>()?;
        let parent_txids: Vec<String> = parts
            .next()
            .unwrap()
            .split(';')
            .map(|txid| txid.to_string())
            .collect::<Vec<String>>();
        transactions.insert(
            txid.clone(),
            Transaction {
                txid,
                fee,
                weight,
                parents: parent_txids,
            },
        );
    }

    // Calculate potential fee and weight for each transaction, including ancestors
    let mut updated_transactions: HashMap<String, Transaction> = HashMap::new();
    for (txid, transaction) in transactions.iter() {
        let mut total_fee = transaction.fee;
        let mut total_weight = transaction.weight;
        for parent in &transaction.parents {
            if let Some(parent_transaction) = transactions.get(parent) {
                total_fee += parent_transaction.fee;
                total_weight += parent_transaction.weight;
            }
        }
        updated_transactions.insert(
            txid.clone(),
            Transaction {
                txid: txid.clone(),
                fee: total_fee,
                weight: total_weight,
                parents: transaction.parents.clone(),
            },
        );
    }

    // Sort transactions by fee/weight ratio in descending order
    let mut sorted_transactions: Vec<&Transaction> = updated_transactions.values().collect();
    sorted_transactions.sort_by(|a, b| {
        if b.fee > a.fee && a.weight < b.weight {
            Ordering::Less
        } else if a.fee > b.fee && b.weight < a.weight {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    // Greedy selection with backtracking to ensure valid block and maximize fee
    let mut block: Vec<String> = Vec::new();
    let mut current_weight = 0;
    while !sorted_transactions.is_empty() && current_weight < max_block_weight {
        let next_transaction = sorted_transactions.pop().unwrap();
        if is_valid_transaction(&block, next_transaction) {
            block.push(next_transaction.txid.clone());
            current_weight += next_transaction.weight;
        } else {
            // Backtrack if necessary
            while !block.is_empty() && !is_valid_transaction(&block, next_transaction) {
                let last_txid = block.pop().unwrap();
                current_weight -= updated_transactions[&last_txid].weight; // Use updated_transactions
            }
        }
    }

    // Print the selected transactions in order
    for txid in block {
        println!("{}", txid);
    }

    Ok(())
}

// fn is_valid_


fn is_valid_transaction(block: &[String], transaction: &Transaction) -> bool {
    // Check if all parents are already in the block
    for parent in &transaction.parents {
        if !block.iter().any(|txid| txid == parent) {
            return false;
        }
    }

    // Check if transaction is not already included
    block.iter().any(|txid| txid == &transaction.txid) == false
}
