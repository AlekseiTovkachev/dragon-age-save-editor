from collections import defaultdict
import sqlite3
import csv
import os


def import_csv_to_sqlite(db_file, csv_file, table_name):
    """
    Imports data from a CSV file into a specified table in an SQLite database.
    If the table already exists, it will be dropped and recreated.

    Args:
        db_file (str): The path to the SQLite database file.
        csv_file (str): The path to the source CSV file.
        table_name (str): The name of the table to create in the database.
    """
    # 1. Check if the CSV file exists before proceeding
    if not os.path.exists(csv_file):
        print(f"❌ Error: The file '{csv_file}' was not found.")
        print("Please make sure the CSV file is in the same directory as this script.")
        return

    try:
        # 2. Connect to the SQLite database. It will be created if it doesn't exist.
        conn = sqlite3.connect(db_file)
        cursor = conn.cursor()
        print(f"🔗 Successfully connected to database '{db_file}'.")

        # 3. Drop the table if it already exists to avoid conflicts or duplicate data
        cursor.execute(f"DROP TABLE IF EXISTS {table_name}")
        print(f"🔪 Dropped existing table '{table_name}', if any.")

        # 4. Define the table schema based on the CSV columns
        # Using TEXT for all columns is safe and flexible for this dataset.
        # The 'id' column is set as the PRIMARY KEY for uniqueness.
        create_table_query = f"""
        CREATE TABLE {table_name} (
            id TEXT PRIMARY KEY,
            name TEXT,
            core_id TEXT,
            tree TEXT,
            type TEXT
        );
        """
        cursor.execute(create_table_query)
        print(
            f"✅ Table '{table_name}' created successfully with columns (id, name, core_id, tree, type)."
        )

        # 5. Open the CSV file and insert its data into the newly created table
        with open(csv_file, "r", encoding="utf-8") as file:
            csv_reader = csv.reader(file)
            header = next(csv_reader)  # Skip the header row

            # Prepare the INSERT statement using parameterized queries to prevent SQL injection
            insert_query = f"INSERT INTO {table_name} (id, name, core_id, tree, type) VALUES (?, ?, ?, ?, ?)"

            # Read each row from the CSV and execute the INSERT statement
            rows_to_insert = list(csv_reader)
            cursor.executemany(insert_query, rows_to_insert)

            print(f"➡️ Inserting {len(rows_to_insert)} rows into the table...")

        # 6. Commit the transaction to save all the changes
        conn.commit()
        print(f"💾 Data committed. {cursor.rowcount} rows were successfully inserted.")

    except sqlite3.Error as e:
        print(f"❌ A database error occurred: {e}")
    except Exception as e:
        print(f"❌ An unexpected error occurred: {e}")
    finally:
        # 7. Always close the connection, whether the import was successful or not
        if conn:
            conn.close()
            print("🚪 Database connection closed.")


def check_for_duplicates(csv_file):
    """
    Reads a CSV file and identifies which IDs in the first column are duplicated.

    Args:
        csv_file (str): The path to the source CSV file.

    Returns:
        list: A list of IDs that appear more than once in the file.
    """
    print("\n--- Checking for duplicate IDs ---")
    id_counts = defaultdict(int)
    try:
        with open(csv_file, "r", encoding="utf-8") as file:
            csv_reader = csv.reader(file)
            header = next(csv_reader)  # Skip header

            for row in csv_reader:
                if row:  # Ensure row is not empty
                    ability_id = row[0]
                    id_counts[ability_id] += 1

        duplicates = [id for id, count in id_counts.items() if count > 1]

        if duplicates:
            print(f"⚠️ Found {len(duplicates)} duplicate ID(s):")
            # Sort for consistent output
            print(", ".join(sorted(duplicates)))
        else:
            print("✅ No duplicate IDs found in the CSV file.")

        return duplicates

    except FileNotFoundError:
        # This case is handled in the main function, but included for completeness
        return []
    except Exception as e:
        print(f"❌ An error occurred during duplicate check: {e}")
        return []


# --- Main execution block ---
if __name__ == "__main__":
    DATABASE_FILE = "gamedata.db"
    CSV_SOURCE_FILE = "abilities_clean.csv"
    TABLE_NAME = "abilities"
    # check_for_duplicates(CSV_SOURCE_FILE)

    print("--- Starting CSV to SQLite Import Process ---")
    import_csv_to_sqlite(DATABASE_FILE, CSV_SOURCE_FILE, TABLE_NAME)
    print("--- Process Finished ---")
