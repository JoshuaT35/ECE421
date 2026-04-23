import os
import requests

# Ranks (use numeric “10” rather than “ten”)
RANKS = ["ace", "king", "queen", "jack", "10", "9", "8", "7", "6", "5", "4", "3", "2"]
SUITS = ["spades", "hearts", "diamonds", "clubs"]

# Base URL of a public GitHub repo containing 52 PNG card images
BASE_URL = "https://raw.githubusercontent.com/hayeah/playing-cards-assets/master/png/"

DEST_DIR = "assets/cards"
os.makedirs(DEST_DIR, exist_ok=True)

def download_card(rank: str, suit: str):
    filename = f"{rank}_of_{suit}.png"
    url = BASE_URL + filename
    resp = requests.get(url)
    if resp.status_code == 200:
        path = os.path.join(DEST_DIR, filename)
        with open(path, "wb") as f:
            f.write(resp.content)
        print(f"✅ Downloaded {filename}")
    else:
        print(f"❌ Failed {filename} (HTTP {resp.status_code})")

def download_back():
    url = BASE_URL + "back.png"
    resp = requests.get(url)
    if resp.status_code == 200:
        path = os.path.join(DEST_DIR, "card_back.png")
        with open(path, "wb") as f:
            f.write(resp.content)
        print("✅ Downloaded card_back.png")
    else:
        print(f"❌ Failed card_back (HTTP {resp.status_code})")

if __name__ == "__main__":
    print("Fetching 52 card images…")
    for rank in RANKS:
        for suit in SUITS:
            download_card(rank, suit)
    download_back()
    print("Done — all images saved in:", DEST_DIR)