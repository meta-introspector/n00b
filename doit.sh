#!/bin/bash

# doit.sh - Mac/Linux Setup Script for N00b Repository

# This script helps set up your development environment to contribute to this repository
# and to get started with interacting with AI APIs or running AI-powered CLI tools.

# IMPORTANT: This script DOES NOT launch the Gemini CLI agent you've been interacting with.
# The Gemini CLI agent is a cloud-based AI service. This script prepares your local system.

echo "--- Starting N00b Repository Setup (Mac/Linux) ---"

# 1. Ensure Homebrew is installed (Mac package manager)
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Checking for Homebrew installation (Mac only)..."
    if ! command -v brew &> /dev/null; then
        echo "Homebrew not found. Installing Homebrew..."
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    fi
    echo "Homebrew is installed."
fi

# 2. Ensure Git is installed
echo "Checking for Git installation..."
if ! command -v git &> /dev/null; then
    echo "Git not found. Installing Git..."
    if [[ "$OSTYPE" == "darwin"* ]]; then
        brew install git
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        sudo apt-get update && sudo apt-get install git -y # For Debian/Ubuntu
        # Add other Linux distro package managers if needed (e.g., yum for Fedora)
    fi
fi
echo "Git is installed."

# 3. Ensure Python and Virtualenv are set up
echo "Checking for Python installation..."
if ! command -v python3 &> /dev/null; then
    echo "Python not found. Installing Python..."
    if [[ "$OSTYPE" == "darwin"* ]]; then
        brew install python
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        sudo apt-get update && sudo apt-get install python3 python3-pip -y
    fi
fi
echo "Python is installed."

# Create and activate a virtual environment
echo "Setting up Python virtual environment..."
if [ ! -d "venv" ]; then
    python3 -m pip install --upgrade pip
    python3 -m pip install virtualenv
    virtualenv venv
fi

echo "Activating virtual environment. Run 'deactivate' to exit."
source venv/bin/activate

echo "--- Environment setup complete. You are now in a Python virtual environment. ---"
echo "You can now install project-specific Python dependencies (e.g., 'pip install -r requirements.txt')."
echo "Refer to cli_setup.md for more details."
