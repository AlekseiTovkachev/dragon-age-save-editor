from typing import List, Optional


class Ability:

    ability_id: int
    name: str
    core_id: List[int]
    tree: str
    ability_type: str

    def __init__(
        self,
        ability_id: int,
        name: str,
        core_id: List[int],
        tree: str,
        ability_type: str,
    ):
        self.ability_id = ability_id
        self.name = name
        self.core_id = core_id
        self.tree = tree
        self.ability_type = ability_type

    def __repr__(self) -> str:
        return (
            f"Ability(ability_id='{self.ability_id}', "
            f"name='{self.name}', ability_type='{self.ability_type}')"
        )
