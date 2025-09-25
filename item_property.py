class ItemProperty:
    """
    Represents a single property or buff on an item, such as '+5 Strength'.
    """
    id: int
    name: str
    power: float
    effect_id: int
    vfx_id: int

    def __init__(
        self,
        prop_id: int,
        name: str = "Unknown Property",
        power: float = 0.0,
        effect_id: int = -1,
        vfx_id: int = -1,
    ):
        """
        Initializes an ItemProperty object, optionally with partial data.
        """
        self.id = prop_id
        self.name = name
        self.power = power
        self.effect_id = effect_id
        self.vfx_id = vfx_id

    def __repr__(self):
        """Provides a clean string representation for printing."""
        return f"{self.name} (ID: {self.id}, Power: {self.power})"
