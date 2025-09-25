import sys
import traceback
from typing import Dict, List
from character import Character
from data_manager import GameDataManager
import gff4
from gff4.datoolset_fields import (
    SAVEGAME_BACKPACK,
    SAVEGAME_MONEY,
    SAVEGAME_PARTYLIST,
    SAVEGAME_PARTYPOOLMEMBERS,
    SAVEGAME_PLAYERCHAR,
    SAVEGAME_PLAYERCHAR_CHAR,
)
from item import Item
from item_editor import load_item_from_struct
from character_editor import load_character, save_character


class SaveEditor:

    data_manager: GameDataManager

    # Raw save data
    save_header: gff4.Header
    save_data: gff4.Structure
    money_amount_source: gff4.UINT32
    main_character_data_source: gff4.Structure
    companions_source: List[gff4.Structure]
    backpack_source: gff4.List

    money: int
    backpack: Dict[int, Item]
    main_character_data: Character
    companions: List[Character]

    def __init__(self, savefile_path):
        self.save_data = None
        self.data_manager = GameDataManager(db_path="gamedata.db")

        print(f"--- Reading save file: {savefile_path} ---")
        try:
            with open(savefile_path, "rb") as f:
                self.save_data, self.save_header = gff4.read_gff4(f)
        except FileNotFoundError:
            raise Exception(f"Save file not found at '{savefile_path}'")

        self.main_character_data_source = self.save_data[SAVEGAME_PLAYERCHAR][
            SAVEGAME_PLAYERCHAR_CHAR
        ]
        self.money_amount_source = self.save_data[SAVEGAME_PARTYLIST][SAVEGAME_MONEY]
        self.companions_source = [
            companion
            for companion in self.save_data[SAVEGAME_PARTYLIST][
                SAVEGAME_PARTYPOOLMEMBERS
            ]
        ]
        self.backpack_source = self.save_data[SAVEGAME_PARTYLIST][SAVEGAME_BACKPACK]

        self.money = int(self.money_amount_source)
        self.backpack = {}
        self.load_backpack_items()
        self.load_main_character()
        self.load_companions()

    def load_backpack_items(self):
        try:
            slot_counter = 0

            for item_struct in self.backpack_source:
                item = load_item_from_struct(item_struct, self.data_manager)
                self.backpack[f"Slot {slot_counter}"] = item
                slot_counter += 1

        except (KeyError, IndexError) as e:
            print(f"Warning: Could not find or parse equipment list. Error: {e}")

    def load_main_character(self):
        self.main_character_data = load_character(
            self.main_character_data_source, self.data_manager, main_character=True
        )

    def load_companions(self):
        self.companions = [
            load_character(companion_source, self.data_manager, main_character=False)
            for companion_source in self.companions_source
        ]

    def commit_changes(self):

        print("--- Committing changes to GFF data structure ---")

        self.save_data[SAVEGAME_PARTYLIST][SAVEGAME_MONEY] = gff4.UINT32(self.money)
        print(f"  - Money updated to: {self.money}")

        # 2. Update Main Character
        save_character(self.main_character_data_source, self.main_character_data)

        # 3. Update Companions
        for companion, companion_source in zip(self.companions, self.companions_source):
            save_character(companion_source, companion)

        # TODO: Add logic to save backpack items if they are modified.

    def save_to_file(self, output_path: str):
        """
        Commits all changes and writes the modified save data to a new file.

        Args:
            output_path (str): The path to save the new .das file to.
        """
        # First, ensure all changes from our Python objects are written back to the GFF data
        self.commit_changes()

        print(f"\n--- Writing save data to: {output_path} ---")
        try:
            with open(output_path, "wb") as f:
                # Use the gff4 library's write function
                gff4.write_gff4(f, self.save_data, self.save_header)
            print("✅ Save successful!")
        except Exception as e:
            print("--- Full Traceback ---")
            traceback.print_exc()
            print("----------------------")


def main():
    save_path = (
        # sys.argv[1] if len(sys.argv) > 1 else "saves_for_testing/updated.das"
        sys.argv[1] if len(sys.argv) > 1 else "saves_for_testing\Camptesting2.das"
    )
    try:
        reader = SaveEditor(save_path)
        reader.money = 99999999
        reader.main_character_data.stats["Willpower"] = 99
        reader.commit_changes()
        reader.save_to_file("saves_for_testing/updated.das")
        print(int(reader.money_amount_source))
    except Exception as e:
        print(f"\n❌ A critical error occurred: {e}")
        print("--- Full Traceback ---")
        traceback.print_exc()
        print("----------------------")
    print("Success")


if __name__ == "__main__":
    main()
