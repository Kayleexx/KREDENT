use anyhow::Result;

pub struct PaymentRequest{
    pub to: String,
    pub amount: u64,
    pub memo: String,
}

pub fn send_shielded(req: PaymentRequest) -> Result<()> {
    println!("Sending {} zats to {}", req.amount, req.to);
    println!("Memo: {}", req.memo);

    // simulate txid
    let fake_txid = "txid_fakedemo1234567890";
    println!("✔ Shielded tx submitted (mock) txid={}", fake_txid);

    Ok(())
}
