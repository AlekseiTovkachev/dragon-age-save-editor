import sys
import sqlite3
import gff4

# Assumes you have refactored datoolset_fields.py as previously discussed
from gff4.datoolset_fields import SAVEGAME_PARTYLIST, SAVEGAME_BACKPACK, TEMPLATERESREF

DATABASE_FILE = "gamedata.db"


def load_names_from_db(game_version="dao"):
    """
    Loads item names for a specific game from the SQLite database into a dictionary.
    """
    item_names = {}
    print(f"--- Loading '{game_version.upper()}' item names from {DATABASE_FILE} ---")
    try:
        con = sqlite3.connect(DATABASE_FILE)
        cur = con.cursor()
        # Query the database for items matching the specified game version
        query = "SELECT resref, name FROM items WHERE game = ?"
        for resref, name in cur.execute(query, (game_version,)):
            item_names[resref] = name
        con.close()
    except sqlite3.OperationalError:
        print(f"Error: Database not found at '{DATABASE_FILE}'.")
        print("Please run the 'import_data_final.py' script first to create it.")
        return None  # Return None to indicate failure

    print(f"--- Loaded {len(item_names)} item names ---\n")
    return item_names


def main(savefile_path):
    """
    Main function to load a save file and print the translated inventory using the database.
    """
    if not savefile_path:
        print("Error: Please provide path to your savegame.das file.")
        return

    # 1. Create the lookup table from the database for DAO items
    item_name_map = load_names_from_db("dao")
    if item_name_map is None:
        return  # Stop if the database couldn't be loaded

    print(f"--- Reading save file: {savefile_path} ---")
    try:
        with open(savefile_path, "rb") as f:
            data, header = gff4.read_gff4(f)
    except Exception as e:
        print(f"An error occurred while reading the save file: {e}")
        return

    # 2. Navigate to the inventory list
    try:
        party_list = data[SAVEGAME_PARTYLIST]
        inventory_list = party_list[SAVEGAME_BACKPACK]
    except KeyError as e:
        print(f"Error: Could not find inventory in the save file. Missing key: {e}")
        return

    print(f"--- Found {len(inventory_list)} items in backpack ---")

    # 3. Loop through items, get the raw name, clean it, and translate it
    for i, item_struct in enumerate(inventory_list):
        try:
            raw_item_object = item_struct[TEMPLATERESREF]

            # Clean the ResRef name by converting to string and stripping nulls
            raw_item_name = str(raw_item_object).rstrip("\x00")

            # Look up the name in our dictionary
            translated_name = item_name_map.get(
                raw_item_name.lower(),
                f"<{raw_item_name}>",  # Fallback for untranslated items
            )

            print(f"  {i+1: >2}. {translated_name}")
        except KeyError:
            print(f"  {i+1: >2}. <Unknown Item Structure>")


if __name__ == "__main__":
    save_path = "Camp.das"
    main(save_path)
