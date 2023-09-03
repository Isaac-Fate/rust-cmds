#!/bin/sh

# Exit bash script on error
set -e

# Name of the built bin file
BUILT_BIN_NAME=cd2

# Name of the bin file
# to put under the local bin dir
BIN_NAME=_cd2

# Local bin dir where
# the bin will reside
LOCAL_BIN_DIR="$HOME"/.local/bin

# Create local bin dir if it does not exist
[ -d "$LOCAL_BIN_DIR" ] || mkdir -p "$LOCAL_BIN_DIR"

# Build the Rust bin file using cargo
echo Compiling from source
cargo build

# Copy the compiled bin to the local bin dir
BUILT_BIN_FILE_PATH=$(realpath ../target/debug/$BUILT_BIN_NAME)
TARGET_BIN_FILE_PATH="$HOME"/.local/bin/$BIN_NAME
echo Copying "$BUILT_BIN_FILE_PATH" to "$TARGET_BIN_FILE_PATH"
cp "$BUILT_BIN_FILE_PATH" "$TARGET_BIN_FILE_PATH"

# Data dir storing 
# the SQLite database files, migration scripts, etc.
DATA_DIR="$HOME"/.cd2

# Create data dir if it does not exist
if ! [ -d "$DATA_DIR" ]; then 
    echo "$DATA_DIR" does not exist
    mkdir -p "$DATA_DIR"
    echo Created "$DATA_DIR"
fi

# Copy the migration scripts
# to data dir
MIGRATIONS_DIR=assets/migrations
TARGET_MIGRATIONS_DIR="$DATA_DIR"/migrations
echo Copying "$MIGRATIONS_DIR" to "$TARGET_MIGRATIONS_DIR"
cp -r $MIGRATIONS_DIR "$TARGET_MIGRATIONS_DIR"

# Create the zsh functions dir
ZSH_FUNCTIONS_DIR=$HOME/.zshfn
if ! [ -d "$ZSH_FUNCTIONS_DIR" ]; then 
    echo "$ZSH_FUNCTIONS_DIR" does not exist
    mkdir -p "$ZSH_FUNCTIONS_DIR"
    echo Created "$ZSH_FUNCTIONS_DIR"
fi

# Add this zsh functions dir to FPATH
# by editing .zshrc
ZSH_RC_FILE_PATH=$HOME/.zshrc
if echo "$FPATH" | grep -q "$ZSH_FUNCTIONS_DIR"; then 
    echo "$ZSH_RC_FILE_PATH" is already in FPATH
else
    LINES="\n"
    LINES="${LINES}# Zsh functions dir\n"
    LINES="${LINES}export FPATH="$ZSH_FUNCTIONS_DIR:\$FPATH""
    echo "$LINES" >> "$ZSH_RC_FILE_PATH"
    echo Added "$ZSH_RC_FILE_PATH" to FPATH
fi

# Copy cd2 bash file (containing the cd2 function)
# to zsh functions dir
ZSH_FUNCTION_NAME=cd2
ZSH_FUNCTION_FILE_PATH=assets/$ZSH_FUNCTION_NAME
cp $ZSH_FUNCTION_FILE_PATH "$ZSH_FUNCTIONS_DIR"/$ZSH_FUNCTION_NAME
echo Copied $ZSH_FUNCTION_FILE_PATH to "$ZSH_FUNCTIONS_DIR"/$ZSH_FUNCTION_NAME

# Load the cd2 function
LOAD_ZSH_FUNCTION_LINE="autoload $ZSH_FUNCTION_NAME"
if grep -q "$LOAD_ZSH_FUNCTION_LINE" "$ZSH_RC_FILE_PATH"; then 
    echo "$LOAD_ZSH_FUNCTION_LINE" is already written in "$ZSH_RC_FILE_PATH"
else
    echo "$LOAD_ZSH_FUNCTION_LINE" >> "$ZSH_RC_FILE_PATH"
    echo Loaded "$ZSH_FUNCTION_NAME"
fi
