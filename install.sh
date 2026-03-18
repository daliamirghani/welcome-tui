#!/bin/bash

GREEN='\033[1;32m'
RED='\033[1;31m'
NC='\033[0m'
BOLD='\033[1m'

sudo apt update -y
sudo apt install -y $@

clear

if [ $? -eq 0 ]; then
    echo -e "${GREEN}${BOLD} SUCCESS ${NC}"
    echo -e "${BOLD}Selected apps installed successfully.${NC}"
else
    echo -e "${RED}${BOLD}[ ERROR ]${NC}"
    echo -e "${BOLD}Something went wrong during installation.${NC}"
fi

echo ""
echo -e "${BOLD}Installation process finished.${NC}"
echo "Closing in 5 seconds..."

sleep 5