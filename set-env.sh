#!/bin/sh

ENV_VALUE="$1"

# Create .env if not exists
[ ! -f .env ] && touch .env

# Remove any existing ENV= line
sed -i.bak '/^ENV=/d' .env && rm -f .env.bak

# Add new ENV= line at the end
echo "ENV=$ENV_VALUE" >> .env
