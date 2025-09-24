from typing import List, Tuple

from item_property import ItemProperty

class Item:
    """
    Represents a single item from a Dragon Age: Origins save file,
    holding its core attributes and properties in a structured way.
    """

    resref: str
    name: str
    object_id: int
    item_cost: int
    item_stacksize: int
    item_level: int
    material: int
    properties: List[ItemProperty]

    def __init__(self):
        """Initializes an empty Item object with default values."""
        self.resref = ""
        self.name = ""
        self.object_id = 0
        self.item_cost = 0
        self.item_stacksize = 1
        self.item_level = 0
        self.material = 0
        self.properties = []

    def __repr__(self):
        """Provides a clean string representation for printing the item."""
        # props_str = "\n    ".join(
        #     [f"{p[1]} (ID: {p[0]}, Power: {p[2]})" for p in self.properties]
        # )
        return (
            f"<Item object_id={self.object_id}>\n"
            f"  ResRef: {self.resref}\n"
            f"  Name: {self.name}\n"
            f"  Level: {self.item_level}\n"
            f"  Stack: {self.item_stacksize}\n"
            f"  Material ID: {self.material}\n"
            f"  Properties:\n    {self.properties}"
        )
