import sys
from typing import Dict, List, Tuple
from ability import Ability
from data_manager import GameDataManager
import gff4
from item import Item
from item_loader import load_item_from_struct
from properties import *
from character import CharacterData
from gff4.datoolset_fields import (
    # Navigation constants to find the player data
    SAVEGAME_PLAYERCHAR,
    SAVEGAME_PLAYERCHAR_CHAR,
    SAVEGAME_CREATURE_STATS,
    # Lists of abilities and items
    SAVEGAME_STATLIST,
    SAVEGAME_EQUIPMENT,
    SAVEGAME_EQUIPMENT_ITEMS,
    SAVEGAME_SKILLLIST,
    SAVEGAME_TALENTLIST,
    SAVEGAME_SPELLLIST,
    # Field IDs within structures
    TEMPLATERESREF,
    SAVEGAME_STATPROPERTY_INDEX,
    SAVEGAME_STATPROPERTY_BASE,
    # Added
    SAVEGAME_OBJECT_NAME,
)

class CharacterReader:
    """Reads and processes player character data from a DAO save file."""

    def __init__(self, savefile_path, data_manager: GameDataManager):
        self.save_data = None
        self.character_sheet = None
        self.character_stats = None
        self.data_manager = data_manager

        print(f"--- Reading save file: {savefile_path} ---")
        try:
            with open(savefile_path, "rb") as f:
                self.save_data, _ = gff4.read_gff4(f)
                print("hello")
            self._navigate_to_character_sheet()
        except FileNotFoundError:
            raise Exception(f"Save file not found at '{savefile_path}'")
        except (KeyError, Exception) as e:
            raise Exception(f"Failed to parse save file or find character data: {e}")

    def _navigate_to_character_sheet(self):
        """Finds the main player character data structure within the save file."""
        player_container = self.save_data[SAVEGAME_PLAYERCHAR]
        self.character_sheet = player_container[SAVEGAME_PLAYERCHAR_CHAR]
        self.character_stats = self.character_sheet[SAVEGAME_CREATURE_STATS]

    def extract_all_data(self):
        """Extracts all known character data and returns it in a simple object."""
        if not self.character_sheet:
            raise Exception("Character sheet not found. Cannot extract data.")

        char_data = CharacterData()

        char_data.name = str(self.character_sheet[SAVEGAME_OBJECT_NAME][1])

        self.get_stats(char_data)
        self.get_equipment(char_data)
        self.get_abilities(char_data)

        return char_data

    def get_stats(self, char_data: CharacterData):
        """Extracts base stats, level, and XP as raw IDs and values."""
        stats = {}
        try:
            stats_container = self.character_sheet[SAVEGAME_CREATURE_STATS]
            stats_list = stats_container[SAVEGAME_STATLIST]

            for stat in stats_list:
                stat_id = int(stat[SAVEGAME_STATPROPERTY_INDEX])
                # print(stat_id)
                base_value = int(stat[SAVEGAME_STATPROPERTY_BASE])
                # print(Properties[stat_id]["stat"])
                # Store the raw stat ID and its base value
                if 1 <= stat_id <= 6:
                    stats[Properties[stat_id]["stat"]] = base_value
                elif stat_id == 15:
                    level = base_value
            char_data.stats = stats
            char_data.level = level
        except KeyError as e:
            print(f"Warning: Could not find stats list. Missing key: {e}")

    def get_equipment(self, char_data: CharacterData):
        """
        Extracts equipped items by creating full Item objects using the
        load_item_from_struct factory function.
        """
        try:
            # Navigate to the list of equipped item structures
            # Note: Dragon Age saves have multiple equipment sets; we assume the first is the active one.
            # equipment_sets = self.character_sheet[SAVEGAME_EQUIPMENT]
            # active_set = equipment_sets[0]
            equipped_items_list = self.character_sheet[SAVEGAME_EQUIPMENT_ITEMS]

            equipment = {}
            slot_counter = 0

            for item_struct in equipped_items_list:
                # Use the factory function to create a complete, detailed Item object.
                # This function handles all the logic for extracting resref, properties, etc.
                item = load_item_from_struct(item_struct, self.data_manager)
                # print(item)
                # Store the populated Item object in the character's equipment dictionary.
                equipment[f"Slot {slot_counter}"] = item
                slot_counter += 1

            char_data.equipment = equipment

        except (KeyError, IndexError) as e:
            print(f"Warning: Could not find or parse equipment list. Error: {e}")

    def get_abilities(self, char_data: CharacterData):
        """Extracts skills, talents, and spells as raw IDs."""
        skills = []
        talents = []
        spells = []
        specializations = []
        try:
            # Skills
            skill_list = self.character_stats[SAVEGAME_SKILLLIST]
            for skill_id in skill_list:
                skills.append(self.data_manager.get_ability(int(skill_id)))

            # Talents
            talent_list = self.character_stats[SAVEGAME_TALENTLIST]
            for talent_id in talent_list:
                talent = self.data_manager.get_ability(int(talent_id))
                if talent.ability_type == "Specialization":
                    specializations.append(talent)
                else:
                    talents.append(self.data_manager.get_ability(int(talent_id)))

            # Spells
            spell_list = self.character_stats[SAVEGAME_SPELLLIST]
            for spell_id in spell_list:
                spell = self.data_manager.get_ability(int(spell_id))
                if spell.ability_type == "Specialization":
                    specializations.append(spell)
                spells.append(spell)
            char_data.skills = skills
            char_data.talents = talents
            char_data.spells = spells
            char_data.specializations = specializations
        except KeyError as e:
            print(f"Warning: Could not find ability lists. Missing key: {e}")

def main():
    """Main entry point for the script."""
    save_path = sys.argv[1] if len(sys.argv) > 1 else "saves_for_testing/DAO.das"
    data_manager = GameDataManager()
    try:
        reader = CharacterReader(save_path, data_manager)
        character_data = reader.extract_all_data()
        print(character_data)
    except Exception as e:
        print(f"\nAn error occurred during processing: {e}")


if __name__ == "__main__":
    main()
