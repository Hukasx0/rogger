#!/bin/bash

if [ -f /etc/os-release ]; then
    . /etc/os-release
    echo "Detected os: $PRETTY_NAME"
    if [[ "$ID" == "debian" || $ID == "ubuntu" || $ID_LIKE == "debian" ]]; then
	sudo apt-get update
	if ! command -v sqlite3 &> /dev/null
	then
	    echo "sqlite db not found, installing..."
	    sudo apt-get install sqlite
	fi

	if ! command -v redis-server &> /dev/null
	then
	    echo "Redis not found, installing..."
	    sudo apt-get install redis-server
	fi    
    elif [[ "$ID" == "fedora" || $ID_LIKE == "fedora" ]]; then
	sudo dnf update
	if ! command -v sqlite3 &> /dev/null
	then
	    echo "sqlite db not found, installing..."
	    sudo dnf install sqlite
	fi
	sudo dnf install sqlite-devel
	if ! command -v redis-server &> /dev/null
	then
	    echo "Redis not found, installing..."
	    sudo dnf install redis
	fi
    elif [[ "$ID" == "arch" || $ID_LIKE == "arch" ]]; then
	sudo pacman -Syu
	if ! command -v sqlite3 &> /dev/null
	then
	    echo "sqlite db not found, installing..."
	    sudo pacman -S --noconfirm sqlite
	fi

	if ! command -v redis-server &> /dev/null
	then
	    echo "Redis not found, installing..."
	    sudo pacman -S --noconfirm redis-server
	fi
    else
	echo "Failed to detect installation for this operating system, try installing manually"
	exit 1
    fi
else
    echo "Failed to detect installation for this operating system, try installing manually"
    exit 1
fi

if [ $1 == "--latest" ]; then
    echo "Getting latest changes from GitHub..."
    git pull
fi

if ! command -v cargo &> /dev/null
then
    echo "Cargo not found, installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi

echo "Building rogger as release"
cargo build --release

echo "Starting Redis in background..."
redis-server --daemonize yes

echo "Running rogger binary"
./target/release/rogger
