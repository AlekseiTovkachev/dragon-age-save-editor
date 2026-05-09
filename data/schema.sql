CREATE TABLE abilities (
    id TEXT NOT NULL,
    name TEXT,
    core_id TEXT,
    tree TEXT,
    type TEXT,
    game TEXT NOT NULL,
    PRIMARY KEY (id, game)
);

CREATE INDEX idx_abilities_game_name ON abilities(game, name);
CREATE INDEX idx_abilities_game_type ON abilities(game, type);

CREATE TABLE item_properties (
    id INTEGER NOT NULL,
    game TEXT NOT NULL,
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
    ability_id INTEGER,
    PRIMARY KEY (id, game)
);

CREATE TABLE items (
    resref TEXT NOT NULL,
    name TEXT,
    game TEXT NOT NULL,
    material_family TEXT,
    material_target TEXT,
    wiki_url TEXT,
    stackable INTEGER NOT NULL DEFAULT 0,
    category TEXT NOT NULL DEFAULT 'uncategorized',
    PRIMARY KEY (resref, game)
);

CREATE TABLE material_codes (
    code INTEGER NOT NULL,
    tier INTEGER NOT NULL,
    name TEXT NOT NULL,
    game TEXT NOT NULL,
    family TEXT,
    target TEXT,
    PRIMARY KEY (code, game)
);

CREATE TABLE properties (
    property TEXT NOT NULL,
    type TEXT,
    game TEXT NOT NULL,
    PRIMARY KEY (property, game)
);
