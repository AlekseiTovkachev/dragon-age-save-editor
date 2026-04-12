#!/usr/bin/env python3
"""Rebuild data/gamedata.db from tracked schema and CSV seed files."""

from __future__ import annotations

import csv
import sqlite3
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DATA_DIR = ROOT / "data"
SEED_DIR = DATA_DIR / "seeds"
DB_PATH = DATA_DIR / "gamedata.db"
SCHEMA_PATH = DATA_DIR / "schema.sql"

SEED_TABLES = [
    ("abilities", "abilities_dao.csv"),
    ("abilities", "abilities_da2.csv"),
    ("item_properties", "item_properties.csv"),
    ("items", "items_dao.csv"),
    ("items", "items_daoa.csv"),
    ("items", "items_da2.csv"),
    ("material_codes", "material_codes.csv"),
    ("properties", "properties.csv"),
]

INTEGER_COLUMNS = {
    "item_properties": {
        "id",
        "name_str_id",
        "ip_type",
        "effect",
        "int0",
        "int1",
        "vfx",
        "base_cost",
        "is_negative",
        "projectile_crust",
        "ability_id",
    },
    "items": {"stackable"},
    "material_codes": {"code", "tier"},
}

REAL_COLUMNS = {
    "item_properties": {"float0", "float1", "proc_chance", "base_duration"},
}


def coerce_value(table: str, column: str, value: str) -> object:
    if value == "":
        return None
    if column in INTEGER_COLUMNS.get(table, set()):
        return int(value)
    if column in REAL_COLUMNS.get(table, set()):
        return float(value)
    return value


def insert_seed_file(conn: sqlite3.Connection, table: str, filename: str) -> None:
    path = SEED_DIR / filename
    with path.open(newline="", encoding="utf-8") as file:
        reader = csv.DictReader(file)
        columns = reader.fieldnames or []
        placeholders = ", ".join("?" for _ in columns)
        sql = f"INSERT INTO {table} ({', '.join(columns)}) VALUES ({placeholders})"
        rows = [
            [coerce_value(table, column, row[column]) for column in columns]
            for row in reader
        ]
    conn.executemany(sql, rows)


def main() -> None:
    if DB_PATH.exists():
        DB_PATH.unlink()
    conn = sqlite3.connect(DB_PATH)
    try:
        conn.executescript(SCHEMA_PATH.read_text(encoding="utf-8"))
        for table, filename in SEED_TABLES:
            insert_seed_file(conn, table, filename)
        conn.commit()
    finally:
        conn.close()


if __name__ == "__main__":
    main()
