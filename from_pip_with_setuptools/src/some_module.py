import requests


def check_internet_access() -> str:
    try:
        response = requests.get("https://google.com").reason
    except requests.exceptions.ConnectionError:
        response = "failed, no route"

    return response
