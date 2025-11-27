# **K R E D E N T**

### **Private Credentials. Zero Knowledge. Fully Trustless.**

```
  _  __  ____    _____   ____    _____   _   _   _____ 
 | |/ / |  _ \  | ____| |  _ \  | ____| | \ | | |_   _|
 | ' /  | |_) | |  _|   | | | | |  _|   |  \| |   | |  
 | . \  |  _ <  | |___  | |_| | | |___  | |\  |   | |  
 |_|\_\ |_| \_\ |_____| |____/  |_____| |_| \_|   |_|  
                                                       
Rust → Groth16 → Mina → Zcash
```

---

## **What is KREDENT?**

**KREDENT** is a zero-knowledge cryptographic toolchain built in Rust that enables users to **prove private information without revealing the underlying data**.

It provides:

* **Groth16 zkSNARK proof generation**
* **Proof & key export for Mina zkApps**
* **Shielded Zcash offline transactions**

No accounts. No trust assumptions. No data exposed.

---

## **Core Features**

| Feature                             | Description                                         |
| ----------------------------------- | --------------------------------------------------- |
| **Generate zkSNARK keys**           | Produce proving & verification keys locally         |
| **Create zero-knowledge proofs**    | Prove knowledge of secret data without revealing it |
| **Export JSON proof artifacts**     | Integrates into Mina zkApps or off-chain flows      |
| **Offline Zcash shielded payments** | Build encrypted private transactions                |
| **Fully offline**                   | No internet connection required                     |
| **Fast & lightweight**              | Pure Rust, arkworks, optimized circuits             |

---

## **Installation**

### **Install via cargo**

```bash
cargo install --path .
```

### **Or build release binary**

```bash
cargo build --release
cp target/release/kredent /usr/local/bin/kredent
```

---

## **Usage**

### Show help

```bash
kredent --help
```

### Generate proving & verifying keys

```bash
kredent generate-keys --out-dir keys/
```

### Generate a proof

```bash
kredent prove --secret 42 --out proof.json
```

### Generate Mina verifier (for future on-chain support)

```bash
kredent generate-contract --out-dir contract/
```

### Create shielded Zcash offline transaction

```bash
kredent pay --to zs1testaddress --amount 1 --memo "kredent-test"
```

### Try everything with automated tester

```bash
./test_kredent.sh
```

---

## **What the test script does**

| Step | Action                             |
| ---- | ---------------------------------- |
| 1    | Generates proving & verifying keys |
| 2    | Generates a zero-knowledge proof   |
| 3    | Generates o1js verifier file stub  |
| 4    | Verifies success output            |

---

## **Project Structure**

```
kredent/
├── src/
│   ├── main.rs                # CLI entry
│   ├── zk.rs                  # Groth16 circuits & proof generation
│   ├── serialization.rs       # proof/key serializers
│   ├── pay.rs                 # shielded Zcash tx builder
│   ├── contract_gen.rs        # o1js verifier file generator
├── test_output/               # Generated proofs + keys
├── test_kredent.sh            # Automated testing script
├── Cargo.toml
└── README.md
```

---

## **Roadmap**

| Stage                                 | Status            |
| ------------------------------------- | ----------------- |
| Groth16 proof generation              | ✔ Complete        |
| JSON verifier export                  | ✔ Complete        |
| Offline shielded Zcash tx             | ✔ Implemented     |

---

## **Why KREDENT matters**

* Users need **privacy without trust**
* Credentials and identity should be prove-able **without exposure**
* zk-proofs unlock **web3 identity, access systems, credential markets & DAO on-ramps**
* Zcash + Mina + zkSNARKs = **future of privacy-preserving computation**

---

## **Built with**

| Component          | Tech                   |
| ------------------ | ---------------------- |
| ZK proving system  | Groth16 (arkworks-rs)  |
| Circuit Field type | BN254 / Fr             |
| Export format      | JSON proof + nullifier |
| Payment system     | Zcash shielded builder |
| Language           | Rust 2024 Edition      |

