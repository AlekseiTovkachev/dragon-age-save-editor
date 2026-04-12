#!/usr/bin/env python3
"""Validate the generated game data database."""

from __future__ import annotations

import re
import sqlite3
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DB_PATH = ROOT / "data" / "gamedata.db"
ITEM_RS = ROOT / "src" / "domain" / "item.rs"

VALID_GAMES = {"dao", "daoa", "da2"}
MOJIBAKE_PATTERNS = ("â", "Ã", "�")

DAO_STACKABLE_CATEGORIES = {
    "consumables.bombs",
    "consumables.food_for_dog",
    "consumables.health_poultices",
    "consumables.injury_kits",
    "consumables.lyrium_potions",
    "consumables.poisons",
    "consumables.potions",
    "consumables.salves_incenses_balms",
    "consumables.traps",
    "consumables.unique",
    "consumables.weapon_coating",
    "crafting.resources",
    "weapons.arrows_bolts",
}

DA2_STACKABLE_CATEGORIES = {
    "consumables.backpacks",
    "consumables.bombs",
    "consumables.poisons",
    "consumables.potions",
    "consumables.tomes",
    "crafting.reagents",
}


def item_category_values() -> set[str]:
    text = ITEM_RS.read_text(encoding="utf-8")
    return set(re.findall(r'=> \("([^"]+)", "[^"]+"\)', text))


def fail(message: str) -> None:
    raise SystemExit(message)


def main() -> None:
    allowed_categories = item_category_values()
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row

    games = {
        row["game"]
        for table in ("abilities", "items", "material_codes", "properties", "item_properties")
        for row in conn.execute(f"SELECT DISTINCT game FROM {table}")
    }
    invalid_games = games - VALID_GAMES
    if invalid_games:
        fail(f"Invalid game values: {sorted(invalid_games)}")

    bad_categories = [
        dict(row)
        for row in conn.execute(
            "SELECT resref, game, category FROM items WHERE category IS NULL OR category = ''"
        )
    ]
    if bad_categories:
        fail(f"Items with missing categories: {bad_categories[:10]}")

    unknown_categories = {
        row["category"]
        for row in conn.execute("SELECT DISTINCT category FROM items")
        if row["category"] not in allowed_categories
    }
    if unknown_categories:
        fail(f"Categories missing from ItemCategory: {sorted(unknown_categories)}")

    daoa_count = conn.execute("SELECT COUNT(*) FROM items WHERE game = 'daoa'").fetchone()[0]
    if daoa_count == 0:
        fail("Expected at least one DAOA item row")

    for row in conn.execute("SELECT resref, game, category, stackable FROM items"):
        expected = False
        if row["game"] in {"dao", "daoa"}:
            expected = row["category"] in DAO_STACKABLE_CATEGORIES
        elif row["game"] == "da2":
            expected = row["category"] in DA2_STACKABLE_CATEGORIES
        if bool(row["stackable"]) != expected:
            fail(
                "Stackability mismatch for "
                f"{row['game']}:{row['resref']} category={row['category']} "
                f"stackable={row['stackable']} expected={int(expected)}"
            )

    for table in ("abilities", "items", "material_codes", "properties", "item_properties"):
        columns = [row["name"] for row in conn.execute(f"PRAGMA table_info({table})")]
        text_columns = [column for column in columns if column not in {"stackable"}]
        for column in text_columns:
            for pattern in MOJIBAKE_PATTERNS:
                found = conn.execute(
                    f"SELECT COUNT(*) FROM {table} WHERE CAST({column} AS TEXT) LIKE ?",
                    (f"%{pattern}%",),
                ).fetchone()[0]
                if found:
                    fail(f"Potential mojibake in {table}.{column}: {pattern}")

    for row in conn.execute("SELECT resref, game, wiki_url FROM items WHERE wiki_url IS NOT NULL"):
        if not str(row["wiki_url"]).startswith(("https://dragonage.fandom.com/", "https://dragonage.miraheze.org/")):
            fail(f"Unexpected wiki URL for {row['game']}:{row['resref']}: {row['wiki_url']}")

    print("gamedata verification passed")


if __name__ == "__main__":
    main()
