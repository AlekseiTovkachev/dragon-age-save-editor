import sqlite3
import os
from typing import Optional

from ability import Ability


class GameDataManager:
    """
    Manages loading and caching of game data
    from a SQLite database using a lazy-loading approach.
    """

    def __init__(self, db_path="gamedata.db"):
        """
        Initializes the manager. Does not load any data initially.

        Args:
            db_path (str): The path to the SQLite database file.
        """
        if not os.path.exists(db_path):
            raise FileNotFoundError(
                f"Database not found at '{db_path}'. "
                "Please run the import script to create it."
            )
        self.db_path = db_path
        self._connection = None  # The database connection is lazy-loaded

        # Caches to store data after it's been loaded once
        self._item_cache = {}
        self._property_cache = {}
        self._ability_cache = {}

    def _connect(self):
        """Establishes a database connection if one doesn't already exist."""
        if self._connection is None:
            print(f"--- (Lazy Load) Opening database connection to {self.db_path} ---")
            self._connection = sqlite3.connect(self.db_path)

    def get_item_name(self, resref, game="dao"):
        """
        Gets the in-game name for an item ResRef.

        First checks the cache. If not found, it queries the database and
        stores the result in the cache for future requests.
        """
        # Clean the resref for consistent lookups
        clean_resref = str(resref).rstrip("\x00").lower()

        # 1. Check the cache first for immediate results
        if clean_resref in self._item_cache:
            return self._item_cache[clean_resref]

        # 2. If not in cache, connect to the DB and query
        self._connect()
        cursor = self._connection.cursor()
        query = "SELECT name FROM items WHERE resref = ? AND game = ?"
        cursor.execute(query, (clean_resref, game))
        result = cursor.fetchone()

        # 3. Store the result in the cache and return it
        if result:
            name = result[0]
            self._item_cache[clean_resref] = name
            return name
        else:
            # If not found, cache the resref itself to prevent future DB lookups
            # for the same missing item.
            fallback_name = f"<{clean_resref}>"
            self._item_cache[clean_resref] = fallback_name
            return fallback_name

    def get_property_name(self, prop_id, game="dao"):
        """
        Gets the in-game name for a property ID.
        Follows the same lazy-loading and caching logic as get_item_name.
        """
        # 1. Check cache
        if prop_id in self._property_cache:
            return self._property_cache[prop_id]

        # 2. Query database if not in cache
        self._connect()
        cursor = self._connection.cursor()
        query = "SELECT property FROM properties WHERE id = ? AND game = ?"
        cursor.execute(query, (prop_id, game))
        result = cursor.fetchone()

        # 3. Cache and return the result
        if result:
            name = result[0]
            self._property_cache[prop_id] = name
            return name
        else:
            fallback_name = f"<Property ID: {prop_id}>"
            self._property_cache[prop_id] = fallback_name
            return fallback_name

    def get_ability(self, ability_id: int) -> Optional[Ability]:
        """
        Gets a fully formed Ability object for a given ability ID.

        First checks the cache. If not found, it queries the database,
        constructs an Ability object, and stores it in the cache.
        Returns None if the ability ID is not found.
        """
        # 1. Check the cache first for an immediate result
        if ability_id in self._ability_cache:
            return self._ability_cache[ability_id]

        # 2. If not in cache, connect to the DB and query
        self._connect()
        cursor = self._connection.cursor()
        # Note: The 'type' column from the DB is mapped to 'ability_type'
        query = "SELECT id, name, core_id, tree, type FROM abilities WHERE id = ?"
        cursor.execute(query, (str(ability_id),))  # Query using ID as a string
        result = cursor.fetchone()

        # 3. Process the result, store it in the cache, and return it
        if result:
            # Unpack the database row
            db_id, name, core_id_str, tree, ability_type = result

            # Convert the comma-separated core_id string into a list of integers
            core_ids = []
            if core_id_str:
                core_ids = [
                    int(cid.strip()) for cid in core_id_str.split(",") if cid.strip()
                ]

            # Create an Ability object
            ability_obj = Ability(
                ability_id=int(db_id),
                name=name,
                core_id=core_ids,
                tree=tree,
                ability_type=ability_type,
            )

            # Cache the newly created object
            self._ability_cache[ability_id] = ability_obj
            return ability_obj
        else:
            # If not found, cache a None value to prevent future DB lookups
            self._ability_cache[ability_id] = None
            return None

    def close(self):
        """Closes the database connection if it's open."""
        if self._connection:
            print("--- Closing database connection. ---")
            self._connection.close()
            self._connection = None


# --- Example Usage ---
if __name__ == "__main__":
    print("--- DEMONSTRATION OF ABILITY LAZY LOADING ---")
    try:
        # Create an instance. No database connection is made yet.
        data_manager = GameDataManager(db_path="gamedata.db")

        print("\nRequesting ability with ID 34 ('Melee Archer')...")
        # The first request will trigger the database connection and query.
        ability1 = data_manager.get_ability(34)
        print(f"Result: {ability1}")
        if ability1:
            print(f"  -> Name: {ability1.name}")
            print(f"  -> Core IDs: {ability1.core_id}")

        print("\nRequesting ability with ID 34 again...")
        # This request is instant, as the result is now in the cache.
        ability2 = data_manager.get_ability(34)
        print(f"Result: {ability2} (from cache)")

        print("\nRequesting an unknown ability (ID 999999)...")
        # This will query the DB, find nothing, and cache the None result.
        unknown_ability = data_manager.get_ability(999999)
        print(f"Result: {unknown_ability}")

        print("\nRequesting the same unknown ability again...")
        # This is now served from the cache.
        unknown_ability2 = data_manager.get_ability(999999)
        print(f"Result: {unknown_ability2} (from cache)")

    except FileNotFoundError as e:
        print(f"\nERROR: {e}")
    finally:
        # It's good practice to close the connection when you're done.
        if "data_manager" in locals() and data_manager:
            data_manager.close()
