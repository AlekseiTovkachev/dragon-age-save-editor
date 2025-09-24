import sqlite3
import os

# --- Configuration ---
DATABASE_FILE = "gamedata.db"
# We now include the game ("dao" or "da2") for each file
FILES_TO_IMPORT = [
    ("gff4/ResRefNames.txt", "items", "latin-1", "dao"),
    ("gff4/ResRefNames2.txt", "items", "latin-1", "da2"),
    ("gff4/StatPropertyNames.txt", "properties", "latin-1", "dao"),
    ("gff4/StatPropertyNames2.txt", "properties", "latin-1", "da2"),
]


def create_database():
    """Reads the raw text files and populates a new SQLite database with a 'game' column."""
    if os.path.exists(DATABASE_FILE):
        os.remove(DATABASE_FILE)
        print(f"Removed old database file: {DATABASE_FILE}")

    con = sqlite3.connect(DATABASE_FILE)
    cur = con.cursor()
    print(f"Created new database: {DATABASE_FILE}")

    # --- Create tables with the new 'game' column ---
    print("Creating tables...")
    cur.execute(
        """
        CREATE TABLE items (
            resref TEXT PRIMARY KEY,
            name TEXT,
            game TEXT NOT NULL
        )
    """
    )
    cur.execute(
        """
        CREATE TABLE properties (
            id INTEGER PRIMARY KEY,
            name TEXT,
            game TEXT NOT NULL
        )
    """
    )

    # --- Import the data, now
