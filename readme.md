# Block Builder challenge

The above codebase is a rust implementation of a bitcoin block builder, the code basically reads transactions from a mempool csv file then builds a block by selecting transactions that are most profitable to the miner and then generate a txt file containing brief block details then the transaction Id's of the transactions included in the block.

## Installation
- Clone the repo.
- cd into the repo directory
- Execute the following command ```cargo build``` then ```cargo run```.

## Note:
Once you run the command ```cargo run``` the codebase is executed and a new file is created, this file contains the complete details of the newly created block. This is necesary because the list of included transactions in the block is so large that it is not possible to display them all on the terminal, so it's more legible to write the new block data into a new file. This file contains breakdown of a simplified block header and the transaction Id's each in a new line.