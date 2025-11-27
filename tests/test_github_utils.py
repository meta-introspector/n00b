"""
Tests and mock functions for GitHub utility interactions.
"""

import unittest
from unittest.mock import patch
import json
import re # Added for the locally defined clean_github_url function

# Assuming github_utils is in the parent directory of tests
# You might need to adjust the import based on your project structure
# from ..scripts.github_utils import some_function # Example relative import

# --- Provided Python Snippets ---

# Defining clean_github_url locally for the test snippet as it was not provided separately
def clean_github_url(url: str) -> str:
    match = re.search(r"https://github.com/([^/]+/[^/]+)/?.*", url)
    if match:
        return f"https://github.com/{match.group(1)}"
    return url

def mock_github_release(version: str):
    """
    Mocks a GitHub release response for testing purposes.

    Args:
        version: The version string for the mock release.

    Returns:
        A dictionary representing a mock GitHub release.
    """
    return {
        "url": f"https://api.github.com/repos/owner/repo/releases/{version}",
        "assets_url": f"https://api.github.com/repos/owner/repo/releases/{version}/assets",
        "upload_url": f"https://uploads.github.com/repos/owner/repo/releases/{version}/assets{{?name,label}}",
        "html_url": f"https://github.com/owner/repo/releases/tag/{version}",
        "id": 12345,
        "tag_name": version,
        "target_commitish": "main",
        "name": f"Release {version}",
        "draft": False,
        "prerelease": False,
        "created_at": "2023-01-01T00:00:00Z",
        "published_at": "2023-01-01T00:00:00Z",
        "assets": [],
        "tarball_url": f"https://api.github.com/repos/owner/repo/tarball/{version}",
        "zipball_url": f"https://api.github.com/repos/owner/repo/zipball/{version}",
        "body": f"Release notes for version {version}"
    }

# --- If you want to integrate test_clean_github_url properly into a test suite ---
class TestGithubUtils(unittest.TestCase):
    def test_clean_github_url_integration(self):
        self.assertEqual(clean_github_url("https://github.com/owner/repo/blob/main/path/to/file.md"), "https://github.com/owner/repo")
        self.assertEqual(clean_github_url("https://github.com/owner/repo/"), "https://github.com/owner/repo")
        self.assertEqual(clean_github_url("https://github.com/owner/repo"), "https://github.com/owner/repo")
        self.assertEqual(clean_github_url("invalid-url"), "invalid-url")

# To run tests from command line: python -m unittest your_test_file.py
