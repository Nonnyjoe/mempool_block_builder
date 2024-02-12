use std::fs;
use std::fs::File;
use std::error::Error;
use std::path::Path;
use std::io::{self, Write};

const MAX_WEIGHT: u128 = 4000000;

#[derive(Debug, Clone)]
struct TransactionData {
    txid: String,
    fee: u128,
    weight: u128,
    parent_txid: String,
    ratio: f64,
}

impl TransactionData {
    // Function to create a new transaction structure
    fn new() -> Self {
        Self {
            txid: String::new(),
            fee: 0,
            weight: 0,
            parent_txid: String::new(),
            ratio: 0.0,
        }
    }

    // Function to populate the transaction structure with data from mempool
    fn populate_transaction(&mut self, fields: Vec<&str>) {
         // Process each field as needed
        if let Some(hash) = fields.get(0) {
             self.txid = hash[1..].to_string();
        }
        if let Some(field1) = fields.get(1) {
            self.fee = field1.parse::<u128>().unwrap();
        }
        if let Some(field2) = fields.get(2) {
            self.weight = field2.parse::<u128>().unwrap();
        }
        if let Some(field3) = fields.get(3) {
            if field3.len() > 1 {
                self.parent_txid = field3[..field3.len() - 1].to_string();
            }
        }
        if let Some(field1) = fields.get(1) {
          if let Some(field2) = fields.get(2) {
            let fee = field1.parse::<u128>().unwrap();
            let weight = field2.parse::<u128>().unwrap();
            self.ratio = weight as f64 / fee as f64;
          }  
        } 
    }
}


// Entry point function, It reads data from the mempool csv, 
// calls the extract function to get all the necessary fields from the transactions.
// Then select transactions based on priority;
fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("./mempool.csv ");
    let content = fs::read_to_string(path)?;
    let all_transactions = extract_fields(content);
    select_priority(all_transactions);
    Ok(())
}

// Function that collects a body of transactions, loops through them and
// returns an array of individual transactions well sorted out in the transaction struct.
fn extract_fields (content: String) -> Vec::<TransactionData> {
    let mut all_transactions: Vec::<TransactionData> = Vec::new();

    // Process each line of the CSV file
    for line in content.lines() {
        // create a new transaction storage.
        let mut transaction = TransactionData::new();

        // Split the line into fields separated by commas
        let fields: Vec<&str> = line.trim().split(',').collect();
        transaction.populate_transaction(fields);
       all_transactions.push(transaction);
    };

    // Sort the vector based on the ratio field in ascending order
    all_transactions.sort_by(|a, b| a.ratio.partial_cmp(&b.ratio).unwrap());
    return all_transactions;
}

// Sort the array of transactions to create a new block in a way that offers the miner best profit.
// It also runs a continious loop over individual transactions to pick up all parents transactions first,
// before child transactions. Then logs the final block and it's header.
fn select_priority(all_transactions: Vec::<TransactionData>) {
    let mut block_weight: u128 = 0;
    let mut new_block_transactions: Vec<TransactionData> = Vec::new();
    let mut miner_fee: u128 = 0;

    // Loop through the array of structs
    for data in &all_transactions {
        if block_weight + data.weight <= MAX_WEIGHT {
             // Check if parent_txid is empty
            let is_parent_empty = data.parent_txid.is_empty();
            if !is_parent_empty {
                find_transaction_parent(data.parent_txid.clone(), &all_transactions, &mut new_block_transactions, &mut block_weight, &mut miner_fee);
            }
            populate_new_block(data, &mut new_block_transactions, &mut block_weight, &mut miner_fee);
        }
    }
    print_new_block(new_block_transactions.clone());
    println!("Block Header =========================");
    println!("number of transaction is:{}", new_block_transactions.len());
    println!("new block weigth is:{}", block_weight);
    println!("new block miner fee is:{}", miner_fee);
    println!("New Block Transactions exceeds terminal limit, please check the new_block.txt file to view all transaction Id's");

    // Write the new block to a file
    let success = print_new_block_to_file(new_block_transactions.clone(), &mut block_weight, &mut miner_fee);
    match success {
        Ok(_) => println!("Block successfully written to file"),
        Err(e) => println!("Error: {}", e)
    }
}

// Functions that runs a loop on the parents of a transaction to add all parents transactions 
// to a block before children transactions
fn find_transaction_parent(transaction_id: String, all_transactions: &[TransactionData], new_block_transactions: &mut Vec<TransactionData>, block_weight: &mut u128, miner_fee: &mut u128) {
    let parent_ids: Vec<&str> = transaction_id.trim().split(';').collect();
    for parent in parent_ids {
        for data in all_transactions {
            if data.txid == parent.to_string() {
                if !data.parent_txid.is_empty(){
                    find_transaction_parent(data.parent_txid.clone(), all_transactions, new_block_transactions, block_weight, miner_fee);
                }
                populate_new_block(data, new_block_transactions, block_weight, miner_fee);
            }
        }
    }
}

// Function to populate a new block array with the necessary transactions that meet the conditions to be added into the block.
fn populate_new_block (single_transaction: &TransactionData , new_block_transactions: &mut Vec<TransactionData>, block_weight: &mut u128, miner_fee: &mut u128) {
    if *block_weight + single_transaction.weight <= MAX_WEIGHT && is_valid_addition(new_block_transactions, single_transaction){
        new_block_transactions.push(single_transaction.clone());   
        *block_weight += single_transaction.weight;
        *miner_fee += single_transaction.fee;
    }
}

// Function that validates a transaction to be sure it meet certain requirements before they can be added into a block.
fn is_valid_addition(new_block_transactions: &mut Vec<TransactionData>, single_transaction: &TransactionData) -> bool {
    let mut is_valid = true;
    for transaction in new_block_transactions.clone() {
        if transaction.txid == single_transaction.txid {
            is_valid = false;
            return is_valid;
        }
    }
    if !single_transaction.parent_txid.is_empty() {
        let parent_ids: Vec<&str> = single_transaction.parent_txid.trim().split(';').collect();
        for parent in parent_ids {
            is_valid = false;
            for transaction in new_block_transactions.clone() {
                if transaction.txid == parent.to_string() {
                    is_valid = true;
                }
            }
            if is_valid == false {
                return is_valid;
            }
        }
    }
    return is_valid;
}

// Function that Logs the entire transaction hash contained in a block in a new line.
fn print_new_block(new_block_transactions: Vec<TransactionData>) {
    for transaction in new_block_transactions {
        println!("{}", transaction.txid);
    }
}

// Function that writes the new block to a .txt file
fn print_new_block_to_file(new_block_transactions: Vec<TransactionData>, block_weight: &mut u128, miner_fee: &mut u128) -> io::Result<()> {
    let mut file = File::create("./new_block.txt")?;

    let _write1 = writeln!(file, "Block Header ====================================");
    let _write1 = writeln!(file, "number of transaction is:{}", new_block_transactions.len());
    let _write1 = writeln!(file, "new block weigth is:{}", block_weight);
    let _write1 = writeln!(file, "new block miner fee is:{}", miner_fee);
    let _write1 = writeln!(file, "Block Transactions ==================================");

    for transaction in new_block_transactions {
        writeln!(file, "{}", transaction.txid)?;
    }

    Ok(())
}


// Block Header===============================
// by ratio {
// new block heigth is:2998
// new block weigth is:3999600
// new block miner fee is:5713878
// }