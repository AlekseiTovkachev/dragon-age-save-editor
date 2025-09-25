import gff4
from gff4.datoolset_fields import (
    SAVEGAME_ITEM_MATERIALTYPE,
    TEMPLATERESREF,
    OBJECT_ID,
    ITEM_COST,
    ITEM_STACKSIZE,
    SAVEGAME_OBJECT_PLOT,
    ITEM_PROPERTIES,
    ITEM_PROPERTY_POWERS,
)
from data_manager import GameDataManager
from item import Item
from item_property import ItemProperty


def load_item_from_struct(
    item_struct: gff4.Structure, data_manager: GameDataManager
) -> Item:
    """
    Factory function to create and populate an Item object from a raw GFF structure.

    Args:
        item_struct: The GFF Structure object for a single item from the save file.
        data_manager: An instance of GameDataManager to look up property names.

    Returns:
        A populated Item object.
    """
    item = Item()

    # --- Extract simple fields, providing default values if a field is missing ---
    item.resref = str(item_struct.get(TEMPLATERESREF, "")).rstrip("\x00")
    item.name = data_manager.get_item_name(item.resref)
    item.object_id = int(item_struct.get(OBJECT_ID, 0))
    item.item_cost = int(item_struct.get(ITEM_COST, 0))
    item.item_stacksize = int(item_struct.get(ITEM_STACKSIZE, 1))
    item.item_level = int(item_struct.get(SAVEGAME_OBJECT_PLOT, 0))
    item.material = int(item_struct.get(SAVEGAME_ITEM_MATERIALTYPE, 0))

    # --- Extract and process the item properties ---
    try:
        property_ids = item_struct[ITEM_PROPERTIES]
        property_powers = item_struct[ITEM_PROPERTY_POWERS]
        # property_effect_ids = item_struct[ITEM_PROPERTY_EFFECTID]
        # property_vfx_ids = item_struct[ITEM_PROPERTY_VFXID]

        # Combine the parallel lists of property IDs and their power levels
        # Not loading the rest of the data of the property, it is needed only while adding or replacing
        for prop_id, prop_power in zip(property_ids, property_powers):
            prop_id = int(prop_id)
            # Use the data manager to get the human-readable name for the property
            prop_name = data_manager.get_item_property_name(prop_id)
            # The power is a float, but stored as an integer bitmask in the file
            # For simplicity, we'll just cast it to float here.
            # A more advanced implementation would convert the bits to a float.
            power_value = float(prop_power)

            property = ItemProperty(
                prop_id=prop_id, name=prop_name, power=power_value
            )

            # item.properties.append((prop_id, prop_name, power_value))
            item.properties.append(property)

    except KeyError:
        # If the item has no properties, these fields won't exist. We can safely ignore this.
        pass

    return item
