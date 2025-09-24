class Item:
    """A simple class to hold data for an in-game item."""

    def __init__(self, resref, name):
        """
        Initializes an Item object.

        Args:
            resref (str): The internal resource reference (e.g., 'gen_im_pot_hea_lhp_01').
            name (str): The user-facing in-game name (e.g., 'Health Poultice').
        """
        self.resref = resref
        self.name = name

    def __repr__(self):
        """Provides a clean string representation for debugging."""
        return f"Item(resref='{self.resref}', name='{self.name}')"
