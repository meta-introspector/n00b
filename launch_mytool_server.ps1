# launch_mytool_server.ps1
#
# This script launches the 'mytool' MCP server in the current PowerShell window.
#
# Before running this script:
# 1. Ensure you are in the root directory of the 'n00b' repository.
# 2. Make sure your GitHub Personal Access Token (PAT) is set as an environment variable
#    in your current PowerShell session (where you run this script from):
#    $env:GITHUB_TOKEN="your_github_pat_here"
#    This token will be used by the 'mytool' server.

Write-Host "Running 'mytool' server in the current PowerShell window..."

# Get the absolute path to the mytool directory
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$mytoolPath = Join-Path $scriptDir "mytool"

# Navigate to the mytool directory and run cargo run
# The GITHUB_TOKEN environment variable from the current session will be inherited.
cd "$mytoolPath"
cargo run

# The server will run in the foreground. Press Ctrl+C to stop it.
Write-Host "Server stopped."
