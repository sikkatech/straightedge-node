#!/bin/bash
db=$1

if [[ "$OSTYPE" == "linux-gnu" ]]; then
  echo "Clearing local data from home dir: $HOME/.local/share/straightedge"
	if [[ "$db" == "staging" ]]; then
		rm -rf ~/.local/share/straightedge/chains/staging_testnet/db/
	elif [[ "$db" == "dev" ]]; then
		rm -rf ~/.local/share/straightedge/chains/dev/db/
		rm -rf ~/.local/share/straightedge/chains/development/db/
	elif [[ "$db" == "straightedge" ]]; then
		rm -rf ~/.local/share/straightedge/chains/straightedge/db/
		rm -rf ~/.local/share/straightedge/chains/straightedge_testnet/db/
	else
		db="all"
		rm -rf ~/.local/share/straightedge/chains/dev/db/
		rm -rf ~/.local/share/straightedge/chains/development/db/
		rm -rf ~/.local/share/straightedge/chains/straightedge/db/
		rm -rf ~/.local/share/straightedge/chains/straightedge_testnet/db/
		rm -rf ~/.local/share/straightedge/chains/staging_testnet/db/
		rm -rf ~/.local/share/straightedge/chains/local_testnet/db/
	fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
  echo "Clearing local data from home dir: $HOME/Library/Application Support/straightedge"
	if [[ "$db" == "staging" ]]; then
		rm -rf ~/Library/Application\ Support/straightedge/chains/staging_testnet/db/
	elif [[ "$db" == "dev" ]]; then
		rm -rf ~/Library/Application\ Support/straightedge/chains/dev/db/
		rm -rf ~/Library/Application\ Support/straightedge/chains/development/db/
	elif [[ "$db" == "straightedge" ]]; then
		rm -rf ~/Library/Application\ Support/straightedge/chains/straightedge/db/
		rm -rf ~/Library/Application\ Support/straightedge/chains/straightedge_testnet/db/
	else
		db="all"
		rm -rf ~/Library/Application\ Support/straightedge/chains/dev/db/
		rm -rf ~/Library/Application\ Support/straightedge/chains/development/db/
		rm -rf ~/Library/Application\ Support/straightedge/chains/straightedge/db/
		rm -rf ~/Library/Application\ Support/straightedge/chains/straightedge_testnet/db/
		rm -rf ~/Library/Application\ Support/straightedge/chains/staging_testnet/db/
		rm -rf ~/Library/Application\ Support/straightedge/chains/local_testnet/db/
	fi
else
  echo "Clearing local data from home dir: $HOME/.local/share/straightedge"
	if [[ "$db" == "staging" ]]; then
		rm -rf ~/.local/share/straightedge/chains/staging_testnet/db/
	elif [[ "$db" == "dev" ]]; then
		rm -rf ~/.local/share/straightedge/chains/dev/db/
		rm -rf ~/.local/share/straightedge/chains/development/db/
	elif [[ "$db" == "straightedge" ]]; then
		rm -rf ~/.local/share/straightedge/chains/straightedge/db/
		rm -rf ~/.local/share/straightedge/chains/straightedge_testnet/db/
	else
		db="all"
		rm -rf ~/.local/share/straightedge/chains/dev/db/
		rm -rf ~/.local/share/straightedge/chains/development/db/
		rm -rf ~/.local/share/straightedge/chains/straightedge/db/
		rm -rf ~/.local/share/straightedge/chains/straightedge_testnet/db/
		rm -rf ~/.local/share/straightedge/chains/staging_testnet/db/
		rm -rf ~/.local/share/straightedge/chains/local_testnet/db/
	fi
fi

echo "Deleted $db databases"
