use blockchain_cli::blockchain::chain::Blockchain;

#[test]
fn test_blockchain_creation() {
    let blockchain = Blockchain::new();
    assert_eq!(blockchain.get_blocks().len(), 1); // Genesis block
}

#[test]
fn test_adding_blocks() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("Test Data 1".to_string());
    blockchain.add_block("Test Data 2".to_string());
    assert_eq!(blockchain.get_blocks().len(), 3);
    assert_eq!(blockchain.get_blocks()[1].data, "Test Data 1");
}
