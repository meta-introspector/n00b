# doit.ps1 - Windows Setup Script for N00b Repository

# This script helps set up your development environment to contribute to this repository
# and to get started with interacting with AI APIs or running AI-powered CLI tools.

# IMPORTANT: This script DOES NOT launch the Gemini CLI agent you've been interacting with.
# The Gemini CLI agent is a cloud-based AI service. This script prepares your local system.

Write-Host "--- Starting N00b Repository Setup (Windows) ---"

# 1. Ensure Chocolatey is installed (as per cli_setup.md)
Write-Host "Checking for Chocolatey installation..."
if (-not (Get-Command choco -ErrorAction SilentlyContinue)) {
    Write-Warning "Chocolatey not found. Please follow steps 1 & 2 in cli_setup.md to install it."
    Write-Warning "You will need to run PowerShell as Administrator."
    # Direct Chocolatey installation via a sub-process that prompts for Admin elevation
    Start-Process powershell -Verb RunAs -ArgumentList "-NoProfile -ExecutionPolicy Bypass -Command 'Start-Process powershell -Verb RunAs -ArgumentList \"-NoProfile -ExecutionPolicy Bypass -Command \\\"Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString(\\\\\\'https://community.chocolatey.org/install.ps1\\\\\"))\\\"\"'"
    Write-Host "Please restart this script after Chocolatey is installed."
    Exit
}
Write-Host "Chocolatey is installed."

# 2. Ensure Git is installed (as per cli_setup.md)
Write-Host "Checking for Git installation..."
if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
    Write-Host "Git not found. Installing Git via Chocolatey..."
    choco install git -y --confirm
}
Write-Host "Git is installed."

# 3. Ensure Python and Virtualenv are set up (as per cli_setup.md)
Write-Host "Checking for Python installation..."
if (-not (Get-Command python -ErrorAction SilentlyContinue)) {
    Write-Host "Python not found. Installing Python via Chocolatey..."
    choco install python -y --confirm
}
Write-Host "Python is installed."

# Create and activate a virtual environment
Write-Host "Setting up Python virtual environment..."
if (-not (Test-Path ".\venv")) {
    python -m pip install --upgrade pip
    python -m pip install virtualenv
    virtualenv venv
}

Write-Host "Activating virtual environment. Run 'deactivate' to exit."
.\venv\Scripts\Activate.ps1

Write-Host "--- Environment setup complete. You are now in a Python virtual environment. ---"
Write-Host "You can now install project-specific Python dependencies (e.g., 'pip install -r requirements.txt')."
Write-Host "Refer to cli_setup.md for more details."
