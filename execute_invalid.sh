#!/bin/bash

# Configuration
BINARY="make asm ARGS="
INPUT_DIR="playground/players_src"



 # Test files
TEST_FILES=(
    "instr_not_exist.s "
    "number_too_big.s"      
    "repated_label.s"
    "invalid_extension.k"   
    "register_too_big.s"
    "not_existing_label.s"  
    "register_too_small.s"
)

# Run each file
for FILE in "${TEST_FILES[@]}"; do
    echo "Running: $BINARY $INPUT_DIR/$FILE"
    $BINARY"$FILE"
    echo ""
done

