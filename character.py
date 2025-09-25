from typing import Dict, List, Optional

from ability import Ability
from item import Item


class Character:
    """A data class to hold all the extracted character data."""

    name: str
    level: int
    stats: Dict[str, int]
    equipment: Dict[int, Item]
    skills: List[Ability]
    talents: List[Ability]
    spells: List[Ability]
    specializations = List[Ability]

    def __init__(
        self,
        name: Optional[str] = None,
        level: Optional[int] = None,
        stats: Optional[Dict[str, float]] = None,
        equipment: Optional[Dict[str, str]] = None,
        skills: Optional[List[Ability]] = None,
        talents: Optional[List[Ability]] = None,
        spells: Optional[List[Ability]] = None,
        specializations: Optional[List[Ability]] = None,
    ):
        """
        Initializes a CharacterData object, allowing partial data to be provided.
        Any omitted fields will receive default empty values.
        """
        self.name = name if name is not None else "Unknown"
        self.level = level if level is not None else 0
        self.stats = stats if stats is not None else {}
        self.equipment = equipment if equipment is not None else {}
        self.skills = skills if skills is not None else []
        self.talents = talents if talents is not None else []
        self.spells = spells if spells is not None else []
        self.specializations = specializations if specializations is not None else []

    def __repr__(self) -> str:
        """Provides a developer-friendly summary of the character's data."""
        return (
            f"CharacterData(name='{self.name}', stats={len(self.stats)}, "
            f"equipment={len(self.equipment)}, skills={len(self.skills)}, "
            f"talents={len(self.talents)}, spells={len(self.spells)})"
        )

    def __str__(self) -> str:
        """Provides a detailed, multi-line string representation for readable printing."""
        # Use a list to efficiently build the final string
        output_parts = [f"Character: {self.name}", f"Level: {self.level}"]

        # Helper function to format dictionary sections
        def format_dict_section(title: str, data: Dict):
            output_parts.append(f"  - {title}:")
            if data:
                for key, value in data.items():
                    output_parts.append(f"    - {key.capitalize()}: {value}")
            else:
                output_parts.append("    - (None)")

        # Helper function to format list sections
        def format_list_section(title: str, data: List[Ability]):
            output_parts.append(f"  - {title}:")
            if data:
                for item in data:
                    output_parts.append(f"    - {item.name}")
            else:
                output_parts.append("    - (None)")

        # Format each section of the character data
        format_dict_section("Stats", self.stats)
        format_dict_section("Equipment", self.equipment)
        format_list_section("Skills", self.skills)
        format_list_section("Talents", self.talents)
        format_list_section("Spells", self.spells)

        return "\n".join(output_parts)
