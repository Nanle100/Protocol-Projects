// The project aims to reinforce your understanding of structs, enums, 
//and pattern matching by creating a system that models and processes transactions with various statuses.


// A transaction struct contains more than what we have here which includes timestamp etc.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Transaction {
    sender: String,
    reciever: String,
    amount: u32,
    status: TransactionStatus,
}

// Our enum Fail can also come with reasons for the failure
#[derive(Debug, Clone, PartialEq)]
enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

// this is a function that returns a transaction struct based on their transaction status. we can add other functionalities to filter transactions.
fn filter_transctions(status:TransactionStatus, transactions: Vec<Transaction>) -> Vec<Transaction> {
        transactions.into_iter().filter(|tx| tx.status == status).collect()
}

fn main(){

    let transactions = vec![
        Transaction {
            sender: "Nanle".to_string(),
            reciever: "Obed".to_string(),
            amount: 25,
            status: TransactionStatus::Pending,
        },

        Transaction {
            sender: "James".to_string(),
            reciever: "Obed".to_string(),
            amount: 200,
            status: TransactionStatus::Confirmed,
        },

        Transaction {
            sender: "Prince".to_string(),
            reciever: "Obed".to_string(),
            amount: 56,
            status: TransactionStatus::Pending,
        },

        Transaction {
            sender: "Tim".to_string(),
            reciever: "Obed".to_string(),
            amount: 56,
            status: TransactionStatus::Pending,
        },

        Transaction {
            sender: "Damian".to_string(),
            reciever: "Obed".to_string(),
            amount: 56,
            status: TransactionStatus::Failed,
        },

        Transaction {
            sender: "Damian".to_string(),
            reciever: "Friday".to_string(),
            amount: 500,
            status: TransactionStatus::Pending,
        },
    ];

    // Filter confirmed transactions
    let confirmed_transactions = filter_transctions(TransactionStatus::Pending, transactions);

    println!("Confirmed transactions:");
    // looping through transaction to print the expected ones
    for tx in confirmed_transactions {
        println!{"{:?}", tx};
    }

}



