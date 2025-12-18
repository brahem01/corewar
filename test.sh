#!/bin/bash

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Directories
REF_DIR="playground/players_src"
YOURS_DIR="players/bin"

# Test files
TEST_FILES=(
    "empty_player.cor"
    "live.cor"
    "pierino_add.cor"
    "pierino_and_ind_ind.cor"
    "pierino_and_ind_reg.cor"
    "pierino_and_reg_ind.cor"
    "pierino_and_reg_reg.cor"
    "pierino_fork.cor"
    "pierino_ldi_dir_dir.cor"
    "pierino_ldi_dir_reg.cor"
    "pierino_ldi_ind_dir.cor"
    "pierino_ldi_ind_reg.cor"
    "pierino_ldi_reg_dir.cor"
    "pierino_ldi_reg_reg.cor"
    "pierino_ld.cor"
    "pierino_lld_dir_reg.cor"
    "pierino_lldi_dir_dir_reg.cor"
    "pierino_lldi_dir_reg_reg.cor"
    "pierino_lldi_ind_dir_reg.cor"
    "pierino_lldi_ind_reg_reg.cor"
    "pierino_lld_ind_reg.cor"
    "pierino_lldi_reg_dir_reg.cor"
    "pierino_lldi_reg_reg_reg.cor"
    "pierino_or_ind_ind.cor"
    "pierino_or_ind_reg.cor"
    "pierino_or_reg_ind.cor"
    "pierino_or_reg_reg.cor"
    "pierino_st_ind.cor"
    "pierino_sti_reg_dir_dir.cor"
    "pierino_sti_reg_dir_reg.cor"
    "pierino_sti_reg_ind_dir.cor"
    "pierino_sti_reg_ind_reg.cor"
    "pierino_sti_reg_reg_dir.cor"
    "pierino_sti_reg_reg_reg.cor"
    "pierino_st_reg.cor"
    "pierino_sub.cor"
    "pierino_test.cor"
    "pierino_xor_ind_ind.cor"
    "pierino_xor_ind_reg.cor"
    "pierino_xor_reg_ind.cor"
    "pierino_xor_reg_reg.cor"
    "zjmp.cor"
)

# Statistics
TOTAL=0
PASSED=0
FAILED=0
MISSING=0

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║          COREWAR BYTECODE COMPARISON TEST SUITE           ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Test each file
for FILE in "${TEST_FILES[@]}"; do
    TOTAL=$((TOTAL + 1))
    REF="$REF_DIR/$FILE"
    YOURS="$YOURS_DIR/$FILE"
    
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}Testing: $FILE${NC}"
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    
    # Check if reference file exists
    if [ ! -f "$REF" ]; then
        echo -e "${RED}✗ Reference file not found: $REF${NC}"
        MISSING=$((MISSING + 1))
        echo ""
        continue
    fi
    
    # Check if your file exists
    if [ ! -f "$YOURS" ]; then
        echo -e "${RED}✗ Your file not found: $YOURS${NC}"
        FAILED=$((FAILED + 1))
        echo ""
        continue
    fi
    
    # File sizes
    echo -e "\n${BLUE}=== File sizes ===${NC}"
    ls -lh "$REF" "$YOURS" | awk '{print $5 "\t" $9}'
    
    # Byte-by-byte comparison
    echo -e "\n${BLUE}=== Byte-by-byte comparison ===${NC}"
    if cmp -s "$REF" "$YOURS"; then
        echo -e "${GREEN}✓ Files are identical!${NC}"
        PASSED=$((PASSED + 1))
    else
        echo -e "${RED}✗ Files differ${NC}"
        FAILED=$((FAILED + 1))
        
        # Show hex diff (first 40 lines)
        echo -e "\n${BLUE}=== Hex diff (first 40 lines) ===${NC}"
        diff -u <(xxd "$REF") <(xxd "$YOURS") | head -40
        
        # Show byte difference count
        echo -e "\n${BLUE}=== Difference summary ===${NC}"
        DIFF_COUNT=$(cmp -l "$REF" "$YOURS" 2>/dev/null | wc -l)
        echo -e "Number of differing bytes: ${RED}$DIFF_COUNT${NC}"
    fi
    
    echo ""
done

# Summary
echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                        TEST SUMMARY                        ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "Total tests:      ${BLUE}$TOTAL${NC}"
echo -e "Passed:           ${GREEN}$PASSED${NC}"
echo -e "Failed:           ${RED}$FAILED${NC}"
echo -e "Missing files:    ${YELLOW}$MISSING${NC}"
echo ""

# Calculate percentage
if [ $TOTAL -gt 0 ]; then
    PERCENTAGE=$((PASSED * 100 / TOTAL))
    echo -e "Success rate:     ${GREEN}$PERCENTAGE%${NC}"
fi

echo ""

# Exit with appropriate code
if [ $FAILED -eq 0 ] && [ $MISSING -eq 0 ]; then
    echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║               ALL TESTS PASSED! 🎉                         ║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
    exit 0
else
    echo -e "${RED}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${RED}║               SOME TESTS FAILED ❌                          ║${NC}"
    echo -e "${RED}╚════════════════════════════════════════════════════════════╝${NC}"
    exit 1
fi
