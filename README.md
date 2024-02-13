# Block Constructor Challenge

This Rust program represents a simple mempool miner that loads transactions from a CSV file, constructs blocks, and mines them based on certain criteria. Below is a breakdown of the code components and functionalities:

## Components

### Miner

- **`struct Miner`**: Represents the miner entity responsible for managing the mempool and finalized transactions.
  - `mempool`: A vector storing transactions in the mempool.
  - `finalized_txids`: A HashSet containing the transaction IDs of finalized transactions.
  - `finalized`: A vector of vectors representing finalized blocks.

- **`impl Miner`**: Defines methods associated with the `Miner` struct.
  - `load_mempool`: Loads transactions from a CSV file into the mempool.
  - `mine`: Mines transactions from the mempool into blocks based on weight constraints and transaction dependencies.

### Transaction

- **`struct Transaction`**: Represents an individual transaction.
  - `txid`: Transaction ID.
  - `fee`: Transaction fee.
  - `weight`: Transaction weight.
  - `parent_txids`: Vector of parent transaction IDs.

- **`impl Transaction`**: Defines methods associated with the `Transaction` struct.
  - `parser`: Parses transaction data from a CSV line.

### Main Function

- **`main`**: Entry point of the program.
  - Loads the mempool from a CSV file.
  - Mines transactions into blocks.
  - Prints finalized blocks and their transactions.

## Usage

1. Clone the repository.
2. Ensure Rust is installed on your system.
3. Navigate to the project directory.
4. Run `cargo build` to build the project.
5. Run the executable generated.

## Example

```bash
$ cargo run mempool.csv
