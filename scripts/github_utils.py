"""
Utility functions for interacting with GitHub, primarily for documentation fetching
and repository searching.
"""

import os
import requests
import json
import re

# --- Provided Python Snippets ---

def github_graphql_request(json_query: dict, github_token: str) -> dict:
    """
    Makes a GraphQL request to the GitHub API.

    Args:
        json_query: A dictionary representing the GraphQL query.
        github_token: Your GitHub personal access token.

    Returns:
        The JSON response from the GitHub API.
    """
    headers = {
        "Authorization": f"Bearer {github_token}",
        "Content-Type": "application/json"
    }
    response = requests.post("https://api.github.com/graphql", headers=headers, json=json_query)
    response.raise_for_status()
    return response.json()

def github_repo_search_rest(query: str, github_token: str) -> dict:
    """
    Searches GitHub repositories using the REST API.

    Args:
        query: The search query string.
        github_token: Your GitHub personal access token.

    Returns:
        The JSON response from the GitHub API.
    """
    headers = {
        "Authorization": f"Bearer {github_token}",
        "Accept": "application/vnd.github.v3+json"
    }
    params = {"q": query}
    response = requests.get("https://api.github.com/search/repositories", headers=headers, params=params)
    response.raise_for_status()
    return response.json()

def resource_to_github_path(resource_type: str, resource_name: str, github_repo_org: str, github_repo_name: str) -> str:
    """
    Converts a resource type and name to a GitHub documentation path.

    Args:
        resource_type: The type of resource (e.g., 'aws_s3_bucket').
        resource_name: The name of the resource.
        github_repo_org: The GitHub organization name.
        github_repo_name: The GitHub repository name.

    Returns:
        The constructed GitHub documentation path.
    """
    cleaned_resource_type = resource_type.replace("aws_", "").replace("awscc_", "")
    # Assuming documentation follows a pattern like:
    # docs/resources/s3_bucket.md for aws_s3_bucket
    # docs/data-sources/s3_bucket.md for aws_s3_bucket data source
    category = "resources" if resource_type.startswith("aws_") else "data-sources" # Simplified assumption
    file_name = f"{cleaned_resource_type}.md"
    return f"https://github.com/{github_repo_org}/{github_repo_name}/blob/main/docs/{category}/{file_name}"

def fetch_github_documentation(
    github_repo_url: str,
    resource_type: str,
    resource_name: str,
    github_token: str
) -> str:
    """
    Fetches documentation content from a GitHub repository based on resource type and name.

    Args:
        github_repo_url: The base URL of the GitHub repository (e.g., "https://github.com/hashicorp/terraform-provider-aws").
        resource_type: The type of resource (e.g., "aws_s3_bucket").
        resource_name: The name of the resource (not directly used for path but for context).
        github_token: Your GitHub personal access token.

    Returns:
        The content of the documentation file as a string.
    """
    org, repo = github_repo_url.replace("https://github.com/", "").split("/", 1)
    doc_path = resource_to_github_path(resource_type, resource_name, org, repo)

    # Clean the doc_path to get the raw content URL
    raw_doc_url = doc_path.replace("/blob/", "/raw/")

    headers = {
        "Authorization": f"Bearer {github_token}",
        "Accept": "application/vnd.github.v3.raw"
    }
    response = requests.get(raw_doc_url, headers=headers)
    response.raise_for_status()  # Raise an exception for HTTP errors
    return response.text

# --- Alternative/Example snippets (included for reference/completeness based on provided context) ---

# Example of resource_to_github_path for specific AWSCC resource
# def resource_to_github_path_awscc_example(r: str) -> str:
#     # Assuming r is like "awscc_s3_bucket"
#     resource_type_parts = r.split('_', 1) # Split only on the first underscore
#     if len(resource_type_parts) < 2:
#         raise ValueError("Invalid resource format")
#     service_prefix = resource_type_parts[0]
#     actual_resource_type = resource_type_parts[1]
#     return f"https://github.com/aws-cloudformation/cloudformation-resource-providers-community/blob/main/docs/{service_prefix}/{actual_resource_type}.md"

# Example of fetch_github_documentation with specific path structure
# def fetch_github_documentation_awscc_example(r: str, resource_name: str, github_token: str) -> str:
#     # Assuming r is like "awscc_s3_bucket"
#     org = "aws-cloudformation"
#     repo = "cloudformation-resource-providers-community"
#     doc_path = resource_to_github_path_awscc_example(r) # Using the example path function
#
#     raw_doc_url = doc_path.replace("/blob/", "/raw/")
#
#     headers = {
#         "Authorization": f"Bearer {github_token}",
#         "Accept": "application/vnd.github.v3.raw"
#     }
#     response = requests.get(raw_doc_url, headers=headers)
#     response.raise_for_status()
#     return response.text
