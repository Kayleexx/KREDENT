#!/usr/bin/env bash
set -e

echo "===================================="
echo "      K R E D E N T   TEST RUN"
echo "===================================="

# Cleanup from previous runs
rm -rf test_output
mkdir test_output

echo "[1] Testing Key Generation..."
cargo run -- generate-keys --out-dir test_output

echo "[DONE] Keys generated."
echo

echo "[2] Testing Proof Generation..."
cargo run -- prove --secret 42 --out test_output/proof.json

cp test_output/vk.json vk.json  # <── ADD THIS FIX

echo "[3] Testing Contract Generation..."
cargo run -- generate-contract --out-dir test_output


echo "[DONE] Contract generated."
echo

echo "[4] Testing Offline Zcash Payment Build..."
cargo run -- pay --to "zs1exampleaddress" --amount 5 --memo "hackathon test"

echo "[DONE] Shielded tx simulated."
echo

echo "[5] Showing Help Output..."
cargo run -- --help

echo "===================================="
echo " ALL TESTS COMPLETED SUCCESSFULLY "
echo "===================================="
