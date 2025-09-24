import sqlite3
import csv
import os

# --- Configuration ---
DATABASE_FILE = "gamedata.db"
CSV_FILE = "data_sources/itemprps.csv"


def import_properties_from_csv():
    """
    Reads the itemprps CSV file and populates a new 'item_properties' table
    in the gamedata.db SQLite database.
    """
    if not os.path.exists(CSV_FILE):
        print(f"Error: The source file '{CSV_FILE}' was not found.")
        print("Please make sure it's in the same directory as this script.")
        return

    if not os.path.exists(DATABASE_FILE):
        print(f"Error: The database '{DATABASE_FILE}' was not found.")
        print("Please run your main database creation script first.")
        return

    try:
        # Connect to the existing database
        con = sqlite3.connect(DATABASE_FILE)
        cur = con.cursor()

        # Drop the table if it already exists for a clean import
        cur.execute("DROP TABLE IF EXISTS item_properties")
        print(f"--- Dropped old 'item_properties' table (if it existed) ---")

        # Create the new table based on the CSV columns
        cur.execute(
            """
            CREATE TABLE item_properties (
                id INTEGER PRIMARY KEY,
                label TEXT,
                name_str_id INTEGER,
                ip_type INTEGER,
                effect INTEGER,
                int0 INTEGER,
                int1 INTEGER,
                float0 REAL,
                float1 REAL,
                condition_script TEXT,
                vfx INTEGER,
                scaling_vector TEXT,
                base_cost INTEGER,
                is_negative INTEGER,
                proc_chance REAL,
                base_duration REAL,
                projectile_crust INTEGER,
                ability_id INTEGER
            )
        """
        )
        print("--- Created new 'item_properties' table ---")

        # Read the CSV file and prepare data for insertion
        properties_to_insert = []
        with open(CSV_FILE, mode="r", encoding="utf-8") as infile:
            reader = csv.reader(infile)
            next(reader)  # Skip the main header row
            next(reader)  # Skip the data type description row

            for row in reader:
                # Clean the data: convert empty strings to None or 0 and cast to correct types
                processed_row = []
                # Column indices that should be integers
                int_indices = {0, 2, 3, 4, 5, 6, 10, 12, 13, 16, 17}
                # Column indices that should be floats/reals
                float_indices = {7, 8, 14, 15}

                for i, value in enumerate(row):
                    clean_value = value.strip()
                    if i in int_indices:
                        processed_row.append(int(clean_value) if clean_value else 0)
                    elif i in float_indices:
                        processed_row.append(float(clean_value) if clean_value else 0.0)
                    else:  # Handle text columns
                        processed_row.append(clean_value if clean_value else None)

                properties_to_insert.append(tuple(processed_row))

        # Use executemany for an efficient bulk insert
        cur.executemany(
            """
            INSERT INTO item_properties VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """,
            properties_to_insert,
        )

        print(
            f"--- Imported {len(properties_to_insert)} records into 'item_properties' ---"
        )

        # Commit changes and close the connection
        con.commit()
        con.close()
        print("--- Database update complete! ---")

    except Exception as e:
        print(f"An error occurred: {e}")


if __name__ == "__main__":
    import_properties_from_csv()
