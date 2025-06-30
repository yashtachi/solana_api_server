#!/bin/bash

# Test script for Solana API Server
echo "Testing Solana API Server..."

# Test health endpoint
echo "Testing health endpoint..."
curl -X GET http://localhost:3000/health

# Test keypair generation
echo "Testing keypair generation..."
curl -X POST http://localhost:3000/keypair \
  -H "Content-Type: application/json"

echo "All tests completed!"
