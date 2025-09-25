from data_manager import GameDataManager
import gff4
from item_editor import load_item_from_struct
from properties import *
from character import Character
from gff4.datoolset_fields import (
    # Lists of abilities and items
    SAVEGAME_OBJECT_NAME,
    SAVEGAME_CREATURE_STATS,
    SAVEGAME_STATLIST,
    SAVEGAME_EQUIPMENT_ITEMS,
    SAVEGAME_SKILLLIST,
    SAVEGAME_TALENTLIST,
    SAVEGAME_SPELLLIST,
    # Field IDs within structures
    TEMPLATERESREF,
    SAVEGAME_STATPROPERTY_INDEX,
    SAVEGAME_STATPROPERTY_BASE,
)


def load_character(
    character_sheet: gff4.Struct, data_manager: GameDataManager, main_character: bool
):

    stats_source = character_sheet[SAVEGAME_CREATURE_STATS]
    equipment_source = character_sheet[SAVEGAME_EQUIPMENT_ITEMS]

    char_data = Character()
    char_data.name = (
        str(character_sheet[SAVEGAME_OBJECT_NAME][1])
        if main_character
        else str(data_manager.get_item_name(character_sheet[TEMPLATERESREF]))
    )

    load_stats(stats_source, char_data)
    load_equipment(equipment_source, char_data, data_manager)
    load_abilities(stats_source, char_data, data_manager)

    return char_data


def load_stats(stats_source: gff4.Struct, char_data: Character):
    """Extracts base stats, level, and XP as raw IDs and values."""
    stats = {}
    try:
        stats_list = stats_source[SAVEGAME_STATLIST]

        for stat in stats_list:
            stat_id = int(stat[SAVEGAME_STATPROPERTY_INDEX])
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


def load_equipment(
    equipment_source: gff4.List, char_data: Character, data_manager: GameDataManager
):

    equipment = {}
    slot_counter = 0

    for item_struct in equipment_source:
        # Use the factory function to create a complete, detailed Item object.
        # This function handles all the logic for extracting resref, properties, etc.
        item = load_item_from_struct(item_struct, data_manager)
        # print(item)
        # Store the populated Item object in the character's equipment dictionary.
        equipment[f"Slot {slot_counter}"] = item
        slot_counter += 1

    char_data.equipment = equipment


def load_abilities(
    stats_source: gff4.Struct,
    char_data: Character,
    data_manager: GameDataManager,
):
    """Extracts skills, talents, and spells as raw IDs."""
    skills = []
    talents = []
    spells = []
    specializations = []
    try:
        # Skills
        skill_list = stats_source[SAVEGAME_SKILLLIST]
        for skill_id in skill_list:
            skills.append(data_manager.get_ability(int(skill_id)))

        # Talents
        talent_list = stats_source[SAVEGAME_TALENTLIST]
        for talent_id in talent_list:
            talent = data_manager.get_ability(int(talent_id))
            if talent == None:
                print(f"Talent not found in the database {talent_id}")
            elif talent.ability_type == "Specialization":
                specializations.append(talent)
            else:
                talents.append(data_manager.get_ability(int(talent_id)))

        # Spells
        spell_list = stats_source[SAVEGAME_SPELLLIST]
        for spell_id in spell_list:
            spell = data_manager.get_ability(int(spell_id))
            if spell == None:
                print(f"Spell not found in the database {spell_id}")
            elif spell.ability_type == "Specialization":
                specializations.append(spell)
            else:
                spells.append(spell)
        char_data.skills = skills
        char_data.talents = talents
        char_data.spells = spells
        char_data.specializations = specializations
    except KeyError as e:
        print(f"Warning: Could not find ability lists. Missing key: {e}")


def save_character(character_sheet: gff4.Structure, char_data: Character):
    """
    Updates the raw GFF character sheet with data from a Character object.

    Args:
        character_sheet: The raw GFF Structure for the character to be modified.
        char_data: The Character object containing the new data.
    """
    print(f"--- Saving data for character: {char_data.name} ---")

    stats_source = character_sheet[SAVEGAME_CREATURE_STATS]

    # Update the character's stats
    save_stats(stats_source, char_data)

    # Update the character's skills, talents, and spells
    save_abilities(stats_source, char_data)

    # Note: Equipment saving is ignored for now
    print(f"--- Character data updated in the GFF structure. ---")


def save_stats(stats_source: gff4.Structure, char_data: Character):
    """Saves the character's attributes (Strength, etc.) and level back to the GFF structure."""
    try:
        stats_list = stats_source[SAVEGAME_STATLIST]

        # Create a reverse mapping from stat name to ID for easy lookup
        # e.g., {"Strength": 1, "Dexterity": 2, ...}
        stats_to_ids = {data["stat"]: prop_id for prop_id, data in Properties.items()}

        # Update existing stats in the save file
        for stat_struct in stats_list:
            stat_id = int(stat_struct[SAVEGAME_STATPROPERTY_INDEX])

            # Look up the name of the current stat
            stat_name = Properties.get(stat_id, {}).get("stat")

            if stat_name in char_data.stats:
                # If this stat from the save file exists in our Character object, update its value
                new_value = char_data.stats[stat_name]
                stat_struct[SAVEGAME_STATPROPERTY_BASE] = gff4.UINT32(new_value)

            elif stat_name == "Level":
                # Handle the character's level separately
                stat_struct[SAVEGAME_STATPROPERTY_BASE] = gff4.UINT32(char_data.level)

    except KeyError as e:
        print(f"Warning: Could not save stats. Missing key: {e}")


def save_abilities(stats_source: gff4.Structure, char_data: Character):
    """Saves the character's skills, talents, and spells back to the GFF structure."""
    try:

        # --- Save Skills ---
        skill_list_source = stats_source[SAVEGAME_SKILLLIST]
        skill_list_source.clear()
        for ability in char_data.skills:
            skill_list_source.append(gff4.UINT32(ability.ability_id))

        # --- Save Talents and Specializations ---
        # Talents and Specializations are stored together in the same list in the save file.
        talent_list_source = stats_source[SAVEGAME_TALENTLIST]
        talent_list_source.clear()
        all_talents = char_data.talents + char_data.specializations
        for ability in all_talents:
            talent_list_source.append(gff4.UINT32(ability.ability_id))

        # --- Save Spells ---
        spell_list_source = stats_source[SAVEGAME_SPELLLIST]
        spell_list_source.clear()
        for ability in char_data.spells:
            spell_list_source.append(gff4.UINT32(ability.ability_id))

    except KeyError as e:
        print(f"Warning: Could not save ability lists. Missing key: {e}")
