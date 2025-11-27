# CLI Environment Setup Guide

This guide provides instructions for setting up essential command-line interface (CLI) tools, including a package manager, version control, and a Python development environment.

## 1. Install Chocolatey (Windows Package Manager)

Chocolatey simplifies software installation on Windows.

1.  **Open PowerShell as Administrator:**
    *   Right-click the Start button.
    *   Select "Windows PowerShell (Admin)" or "Windows Terminal (Admin)".

2.  **Allow PowerShell to run scripts (if not already enabled):**
    ```powershell
    Set-ExecutionPolicy Bypass -Scope Process -Force
    ```

3.  **Install Chocolatey:**
    ```powershell
    Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
    ```
    *   Wait for the installation to complete. You might see some warnings; usually, these can be ignored.
    *   Close and reopen your PowerShell window as Administrator for the changes to take effect.

4.  **Verify installation:**
    ```powershell
    choco -v
    ```
    This should display the Chocolatey version number.

## 2. Install Git (Version Control System)

Git is essential for tracking code changes and collaborating on projects. We'll install it using Chocolatey.

1.  **Open PowerShell as Administrator.**

2.  **Install Git:**
    ```powershell
    choco install git -y
    ```
    *   The `-y` flag automatically confirms any prompts during installation.
    *   Wait for the installation to complete.

3.  **Verify installation:**
    ```powershell
    git --version
    ```
    This should display the Git version number.

## 3. Set up a Python Development Environment (for AI/CLI Tools)

This section guides you through setting up a Python environment often used for AI-related CLI tools and API interactions. *Note: This does not install the Gemini CLI agent itself, but prepares your system for Python-based development.*

1.  **Open PowerShell as Administrator.**

2.  **Install Python (if not already installed):**
    ```powershell
    choco install python -y
    ```
    *   This will typically install the latest stable version of Python.

3.  **Verify Python installation:**
    ```powershell
    python --version
    ```

4.  **Install `pip` (Python package installer) - usually comes with Python:**
    ```powershell
    python -m ensurepip --default-pip
    ```

5.  **Verify `pip` installation:**
    ```powershell
    pip --version
    ```

6.  **Install `virtualenv` (for isolated Python environments):**
    ```powershell
    pip install virtualenv
    ```

7.  **Create a new virtual environment for your project:**
    *   Navigate to your project directory (e.g., `cd C:\Users\YourUser\my_project`).
    *   Create the virtual environment:
        ```powershell
        virtualenv venv
        ```
        (You can replace `venv` with your preferred environment name)

8.  **Activate the virtual environment:**
    ```powershell
    .\venv\Scripts\Activate.ps1
    ```
    *   Your PowerShell prompt should now show `(venv)` indicating the environment is active.

9.  **Install project dependencies within the virtual environment:**
    ```powershell
    pip install -r requirements.txt
    # or install specific packages:
    pip install requests pandas
    ```

10. **Deactivate the virtual environment:**
    ```powershell
    deactivate
    ```

---
