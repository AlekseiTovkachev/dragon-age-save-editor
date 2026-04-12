// Static frontend catalogs exposed through app commands.

pub(crate) struct StaticPlotBooleanFlag {
    pub(crate) id: u16,
    pub(crate) name: &'static str,
    pub(crate) description: &'static str,
    pub(crate) category: &'static str,
}

pub(crate) struct StaticPlotIntegerFlag {
    pub(crate) id: u16,
    pub(crate) name: &'static str,
    pub(crate) description: &'static str,
    pub(crate) category: &'static str,
    pub(crate) options: &'static [StaticPlotIntegerOption],
}

pub(crate) struct StaticPlotIntegerOption {
    pub(crate) value: i32,
    pub(crate) label: &'static str,
}

const HERO_GENDER_OPTIONS: &[StaticPlotIntegerOption] = &[
    StaticPlotIntegerOption {
        value: 1,
        label: "Male",
    },
    StaticPlotIntegerOption {
        value: 2,
        label: "Female",
    },
];

const HERO_RACE_OPTIONS: &[StaticPlotIntegerOption] = &[
    StaticPlotIntegerOption {
        value: 1,
        label: "Dwarf",
    },
    StaticPlotIntegerOption {
        value: 2,
        label: "Elf",
    },
    StaticPlotIntegerOption {
        value: 3,
        label: "Human",
    },
];

pub(crate) const DA2_PLOT_INTEGER_FLAGS: &[StaticPlotIntegerFlag] = &[
    StaticPlotIntegerFlag {
        id: 1000,
        name: "DAO_HERO_GENDER",
        description: "Hero gender",
        category: "Hero",
        options: HERO_GENDER_OPTIONS,
    },
    StaticPlotIntegerFlag {
        id: 1001,
        name: "DAO_HERO_RACE",
        description: "Hero race",
        category: "Hero",
        options: HERO_RACE_OPTIONS,
    },
];

macro_rules! da2_plot_bool {
    ($id:expr, $name:expr, $description:expr, $category:expr) => {
        StaticPlotBooleanFlag {
            id: $id,
            name: $name,
            description: $description,
            category: $category,
        }
    };
}

pub(crate) const DA2_PLOT_BOOLEAN_FLAGS: &[StaticPlotBooleanFlag] = &[
    da2_plot_bool!(2000, "GEN_BACK_CIRCLE", "Origin: Circle mage", "Hero"),
    da2_plot_bool!(
        2001,
        "GEN_BACK_DWARF_COMMONER",
        "Origin: Dwarf commoner",
        "Hero"
    ),
    da2_plot_bool!(2002, "GEN_BACK_DWARF_NOBLE", "Origin: Dwarf noble", "Hero"),
    da2_plot_bool!(2003, "GEN_BACK_ELF_CITY", "Origin: City elf", "Hero"),
    da2_plot_bool!(2004, "GEN_BACK_ELF_DALISH", "Origin: Dalish elf", "Hero"),
    da2_plot_bool!(2005, "GEN_BACK_HUMAN_NOBLE", "Origin: Human noble", "Hero"),
    da2_plot_bool!(
        2006,
        "ARL_REMOVE_DEMON_ISOLDE_KILLS_CONNOR",
        "Isolde kills Connor",
        "Redcliffe"
    ),
    da2_plot_bool!(
        2007,
        "ARL_REMOVE_DEMON_CONNOR_FREED",
        "Connor lives",
        "Redcliffe"
    ),
    da2_plot_bool!(
        2008,
        "ARL_REMOVE_DEMON_ISOLDE_KNOCKED_OUT",
        "Connor dies",
        "Redcliffe"
    ),
    da2_plot_bool!(
        2009,
        "ARL_REMOVE_DEMON_PC_REFUSED_TO_LET_ISOLDE_KILL_CONNOR",
        "Warden refused to let Isolde kill Connor",
        "Redcliffe"
    ),
    da2_plot_bool!(
        2010,
        "ARL_FADE_DEMON_INTIMIDATED",
        "Demon was intimidated in the Fade",
        "Redcliffe"
    ),
    da2_plot_bool!(
        2011,
        "ARL_FADE_OFFER_ACCEPTED_FROM_DEMON",
        "Demon's offer was accepted in the Fade",
        "Redcliffe"
    ),
    da2_plot_bool!(2012, "MAGES_IN_ARMY", "Mages recruited", "Broken Circle"),
    da2_plot_bool!(
        2013,
        "TEMPLARS_IN_ARMY",
        "Templars recruited",
        "Broken Circle"
    ),
    da2_plot_bool!(
        2014,
        "GENITIVI_RETURNS_TO_DENERIM",
        "Andraste's ashes revealed",
        "Urn of Sacred Ashes"
    ),
    da2_plot_bool!(
        2015,
        "NTB_MAIN_ELVES_PROMISED_ALLIANCE",
        "Dalish recruited",
        "Nature of the Beast"
    ),
    da2_plot_bool!(
        2016,
        "NTB_MAIN_WEREWOLVES_PROMISED_ALLIANCE",
        "Werewolves recruited",
        "Nature of the Beast"
    ),
    da2_plot_bool!(
        2017,
        "NTB_MAIN_ZATHRIAN_SACRIFICES_HIMSELF",
        "Zathrian sacrificed himself",
        "Nature of the Beast"
    ),
    da2_plot_bool!(
        2018,
        "ORZ_MAIN___PLOT_04_COMPLETED_KING_IS_BHELEN",
        "Bhelen crowned king",
        "Orzammar"
    ),
    da2_plot_bool!(
        2019,
        "ORZ_MAIN___PLOT_04_COMPLETED_KING_IS_HARROWMONT",
        "Harrowmont crowned king",
        "Orzammar"
    ),
    da2_plot_bool!(
        2020,
        "LANDSMEET_ALISTAIR_ENGAGED_TO_ANORA",
        "Alistair and Anora rule together",
        "Landsmeet"
    ),
    da2_plot_bool!(
        2021,
        "LANDSMEET_ALISTAIR_IS_KING",
        "Alistair rules",
        "Landsmeet"
    ),
    da2_plot_bool!(
        2022,
        "LANDSMEET_ALISTAIR_LEAVES_FOREVER",
        "Alistair is exiled",
        "Landsmeet"
    ),
    da2_plot_bool!(
        2023,
        "LANDSMEET_ALISTAIR_KILLED",
        "Alistair was executed",
        "Landsmeet"
    ),
    da2_plot_bool!(
        2024,
        "LANDSMEET_PLAYER_IS_KING",
        "Warden becomes king",
        "Landsmeet"
    ),
    da2_plot_bool!(
        2025,
        "LANDSMEET_LOGHAIN_KILLED",
        "Loghain dies",
        "Landsmeet"
    ),
    da2_plot_bool!(
        2026,
        "LANDSMEET_ALISTAIR_ENGAGED_TO_PLAYER",
        "Alistair rules with the Warden",
        "Landsmeet"
    ),
    da2_plot_bool!(2027, "LANDSMEET_ANORA_IS_QUEEN", "Anora rules", "Landsmeet"),
    da2_plot_bool!(
        2028,
        "CLIMAX_ARCHDEMON_ALISTAIR_KILLS_ARCHDEMON",
        "Alistair kills the Archdemon",
        "Finale"
    ),
    da2_plot_bool!(
        2029,
        "CLIMAX_ARCHDEMON_LOGHAIN_KILLS_ARCHDEMON",
        "Loghain kills the Archdemon",
        "Finale"
    ),
    da2_plot_bool!(
        2030,
        "CLIMAX_ARCHDEMON_PC_KILLS_ARCHDEMON",
        "Warden kills the Archdemon",
        "Finale"
    ),
    da2_plot_bool!(
        2031,
        "EPI_REWARD_CHANCELLOR",
        "Warden became chancellor",
        "Epilogue"
    ),
    da2_plot_bool!(
        2032,
        "EPI_REWARD_CIRCLE",
        "Circle received the boon",
        "Epilogue"
    ),
    da2_plot_bool!(
        2033,
        "EPI_REWARD_DALISH",
        "Dalish received the land boon",
        "Epilogue"
    ),
    da2_plot_bool!(
        2034,
        "POST_MORRIGAN_RITUAL_WITH_ALISTAIR",
        "Morrigan's ritual was performed with Alistair",
        "Morrigan Ritual"
    ),
    da2_plot_bool!(
        2035,
        "POST_MORRIGAN_RITUAL_WITH_PLAYER",
        "Morrigan's ritual was performed with the Warden",
        "Morrigan Ritual"
    ),
    da2_plot_bool!(
        2036,
        "POST_MORRIGAN_RITUAL_WITH_LOGHAIN",
        "Morrigan's ritual was performed with Loghain",
        "Morrigan Ritual"
    ),
    da2_plot_bool!(2037, "GEN_DOG_RECRUITED", "Dog was recruited", "Companions"),
    da2_plot_bool!(
        2038,
        "GEN_LELIANA_RECRUITED",
        "Leliana was recruited",
        "Companions"
    ),
    da2_plot_bool!(
        2039,
        "GEN_ZEVRAN_RECRUITED",
        "Zevran was recruited",
        "Companions"
    ),
    da2_plot_bool!(
        2040,
        "APP_ALISTAIR_ROMANCE_ACTIVE",
        "Alistair romance active",
        "Romance"
    ),
    da2_plot_bool!(
        2041,
        "APP_ALISTAIR_MAKE_LOVE",
        "Alistair slept with the Warden",
        "Romance"
    ),
    da2_plot_bool!(
        2042,
        "APP_LELIANA_ROMANCE_ACTIVE",
        "Leliana romance active",
        "Romance"
    ),
    da2_plot_bool!(
        2043,
        "APP_LELIANA_MAKE_LOVE",
        "Leliana slept with the Warden",
        "Romance"
    ),
    da2_plot_bool!(
        2044,
        "LELIANA_MAIN_LEAVES_LOTHERING_FOREVER",
        "Leliana was not recruited in Lothering",
        "Companions"
    ),
    da2_plot_bool!(
        2045,
        "LELIANA_MAIN_ATTACKS_PC",
        "Leliana attacked the Warden",
        "Companions"
    ),
    da2_plot_bool!(
        2046,
        "APP_MORRIGAN_ROMANCE_ACTIVE",
        "Morrigan romance active",
        "Romance"
    ),
    da2_plot_bool!(
        2047,
        "APP_MORRIGAN_MAKE_LOVE",
        "Morrigan slept with the Warden",
        "Romance"
    ),
    da2_plot_bool!(
        2048,
        "APP_ZEVRAN_ROMANCE_ACTIVE",
        "Zevran romance active",
        "Romance"
    ),
    da2_plot_bool!(
        2049,
        "APP_ZEVRAN_MAKE_LOVE",
        "Zevran slept with the Warden",
        "Romance"
    ),
    da2_plot_bool!(
        2050,
        "ZEVRAN_MAIN_GOES_HOSTILE",
        "Zevran turned hostile",
        "Companions"
    ),
    da2_plot_bool!(
        2051,
        "ZEVRAN_MAIN_LEAVES_PARTY_AND_GOES",
        "Zevran left the party",
        "Companions"
    ),
    da2_plot_bool!(
        2052,
        "ISABELA_AND_ALISTAIR_THREESOME",
        "Isabela and Alistair had a threesome",
        "Romance"
    ),
    da2_plot_bool!(
        2053,
        "ISABELA_AND_LELIANA_THREESOME",
        "Isabela and Leliana had a threesome",
        "Romance"
    ),
    da2_plot_bool!(
        2054,
        "ISABELA_AND_ZEVRAN_THREESOME",
        "Isabela and Zevran had a threesome",
        "Romance"
    ),
    da2_plot_bool!(
        2055,
        "ISABELA_IN_FOURSOME",
        "Isabela joined a foursome",
        "Romance"
    ),
    da2_plot_bool!(
        2056,
        "ISABELA_SLEPT_WITH",
        "Isabela slept with the Warden",
        "Romance"
    ),
    da2_plot_bool!(
        2057,
        "GXA_BACK_ORLESIAN_WARDEN",
        "Orlesian Warden-Commander",
        "Awakening"
    ),
    da2_plot_bool!(
        2058,
        "VGK_SIEGE___PLOT_09_COMPLETED",
        "Vigil's Keep saved",
        "Awakening"
    ),
    da2_plot_bool!(
        2059,
        "AOA_SIEGE_AMARANTHINE_SAVED",
        "Amaranthine saved",
        "Awakening"
    ),
    da2_plot_bool!(
        2060,
        "VGK_DEFENSES_ROADS_CHOSEN",
        "Protected the trade routes",
        "Awakening"
    ),
    da2_plot_bool!(
        2061,
        "VGK_DEFENSES_FARMS_CHOSEN",
        "Protected the farms",
        "Awakening"
    ),
    da2_plot_bool!(
        2062,
        "VGK_HERREN_COMPLETED_SILVERITE",
        "Vigil's Keep had upgraded silverite walls",
        "Awakening"
    ),
    da2_plot_bool!(
        2063,
        "LTM_MAIN___PLOT_03_KILLED_ARCHITECT",
        "Architect dies",
        "Awakening"
    ),
    da2_plot_bool!(
        2064,
        "GXA_ANDERS_RECRUITED",
        "Anders was recruited",
        "Awakening"
    ),
    da2_plot_bool!(
        2065,
        "GXA_NATHANIEL_RECRUITED",
        "Nathaniel was recruited",
        "Awakening"
    ),
    da2_plot_bool!(
        2066,
        "COD_CHA_ANDERS_DIED_IN_VGK_SIEGE",
        "Anders died during the siege of Vigil's Keep",
        "Awakening"
    ),
    da2_plot_bool!(
        2067,
        "GWB_MAIN_SOPHIA_KILLED",
        "Sophia was killed",
        "Warden's Keep"
    ),
    da2_plot_bool!(
        2068,
        "GWB_MAIN_AVERNUS_KILLED",
        "Avernus was killed",
        "Warden's Keep"
    ),
    da2_plot_bool!(
        2069,
        "GWB_MAIN_COMPLETED",
        "Warden's Keep completed",
        "Warden's Keep"
    ),
    da2_plot_bool!(
        2070,
        "GWB_AVERNUS_DOING_BAD_EXPERIMENTS",
        "Avernus continued unethical experiments",
        "Warden's Keep"
    ),
    da2_plot_bool!(
        2071,
        "GWB_AVERNUS_DOING_GOOD_EXPERIMENTS",
        "Avernus continued ethical research",
        "Warden's Keep"
    ),
    da2_plot_bool!(
        2072,
        "KCC_CAILAN_BURNT",
        "Cailan's corpse was burned",
        "Return to Ostagar"
    ),
    da2_plot_bool!(
        2073,
        "KCC_CAILAN_LEFT",
        "Cailan's corpse was left to the darkspawn",
        "Return to Ostagar"
    ),
    da2_plot_bool!(
        2074,
        "KCC_CAILAN_WOLVES",
        "Cailan's corpse was fed to the wolves",
        "Return to Ostagar"
    ),
    da2_plot_bool!(
        2075,
        "SHL_LEAVES",
        "Shale left the party",
        "The Stone Prisoner"
    ),
    da2_plot_bool!(
        2076,
        "SHL_FOLLOWS",
        "Shale was recruited and survived",
        "The Stone Prisoner"
    ),
    da2_plot_bool!(
        2077,
        "SHL_ATTACKS",
        "Shale attacked the Warden",
        "The Stone Prisoner"
    ),
    da2_plot_bool!(
        2078,
        "STR_MORRIGAN_LEFT",
        "Witch Hunt: let Morrigan go through the Eluvian",
        "Witch Hunt"
    ),
    da2_plot_bool!(
        2079,
        "STR_MORRIGAN_FOLLOWED",
        "Witch Hunt: followed Morrigan through the Eluvian",
        "Witch Hunt"
    ),
    da2_plot_bool!(
        2080,
        "STR_MORRIGAN_STABBED",
        "Witch Hunt: stabbed Morrigan",
        "Witch Hunt"
    ),
    da2_plot_bool!(
        2081,
        "ORZ_ANVIL___PLOT_08_COMPLETED_BRANKA_ALIVE",
        "Support Branka, Anvil saved",
        "Orzammar"
    ),
    da2_plot_bool!(
        2082,
        "ORZ_ANVIL___PLOT_08_COMPLETED_BRANKA_SUICIDES",
        "Support Branka, Anvil destroyed",
        "Orzammar"
    ),
    da2_plot_bool!(
        2083,
        "ORZ_ANVIL___PLOT_08_COMPLETED_CARIDIN",
        "Support Caridin, Anvil destroyed",
        "Orzammar"
    ),
    da2_plot_bool!(
        2084,
        "COD_CHA_NATHANIEL_DIED_IN_VGK_SIEGE",
        "Nathaniel died during the siege of Vigil's Keep",
        "Awakening"
    ),
    da2_plot_bool!(
        2085,
        "GXA_APP_NATHANIEL_FRIENDLY_ELIGIBLE",
        "Nathaniel remained friendly with the Warden-Commander",
        "Awakening"
    ),
    da2_plot_bool!(
        2086,
        "ARL_REMOVE_DEMON_JOWAN_DOES_RITUAL",
        "Jowan performed the ritual to enter the Fade",
        "Redcliffe"
    ),
    da2_plot_bool!(
        2087,
        "ARL_REMOVE_DEMON_CIRCLE_DOES_RITUAL",
        "Circle mages performed the ritual to enter the Fade",
        "Redcliffe"
    ),
    da2_plot_bool!(2088, "ORIGINS_STARTED", "Origins started", "Campaigns"),
    da2_plot_bool!(2089, "ORIGINS_COMPLETED", "Origins completed", "Campaigns"),
    da2_plot_bool!(2090, "AWAKENING_STARTED", "Awakening started", "Campaigns"),
    da2_plot_bool!(
        2091,
        "AWAKENING_COMPLETED",
        "Awakening completed",
        "Awakening"
    ),
    da2_plot_bool!(2092, "STR_STARTED", "Witch Hunt started", "Witch Hunt"),
    da2_plot_bool!(2093, "STR_COMPLETED", "Witch Hunt completed", "Witch Hunt"),
    da2_plot_bool!(
        2094,
        "GWB_STARTED",
        "Warden's Keep started",
        "Warden's Keep"
    ),
    da2_plot_bool!(
        2095,
        "GWB_COMPLETED",
        "Warden's Keep add-in completed",
        "Warden's Keep"
    ),
    da2_plot_bool!(
        2096,
        "LTM_MAIN___PLOT_03_DEAL_WITH_ARCHITECT",
        "Architect lives",
        "Awakening"
    ),
    da2_plot_bool!(
        2097,
        "LANDSMEET_LOGHAIN_LIVES",
        "Loghain lives",
        "Landsmeet"
    ),
    da2_plot_bool!(
        2098,
        "ARL_SIEGE_SIEGE_OVER",
        "Redcliffe siege completed",
        "Redcliffe"
    ),
    da2_plot_bool!(
        2099,
        "ARL_SIEGE_PREP_VILLAGE_ABANDONED",
        "Redcliffe village was abandoned",
        "Redcliffe"
    ),
    da2_plot_bool!(
        2100,
        "MORRIGAN_RITUAL_WITH_ALISTAIR",
        "Morrigan's ritual was performed with Alistair",
        "Morrigan Ritual"
    ),
    da2_plot_bool!(
        2101,
        "MORRIGAN_RITUAL_WITH_LOGHAIN",
        "Morrigan's ritual was performed with Loghain",
        "Morrigan Ritual"
    ),
    da2_plot_bool!(
        2102,
        "MORRIGAN_RITUAL_WITH_PLAYER",
        "Morrigan's ritual was performed with the Warden",
        "Morrigan Ritual"
    ),
    da2_plot_bool!(
        2103,
        "GIB_STARTED",
        "Golems of Amgarrak started",
        "Golems of Amgarrak"
    ),
    da2_plot_bool!(
        2104,
        "RITUAL_PERFORMED",
        "Morrigan's ritual accepted",
        "Morrigan Ritual"
    ),
    da2_plot_bool!(
        2105,
        "ZEVRAN_MAIN_START_AMBUSH_FIGHT_ZEVRAN_ENEMY",
        "Zevran fought the Warden at the ambush",
        "Companions"
    ),
    da2_plot_bool!(
        2106,
        "ZEVRAN_MAIN_KILLED_BEFORE_INTRODUCTION",
        "Zevran died before recruitment",
        "Companions"
    ),
    da2_plot_bool!(
        2107,
        "ZEVRAN_MAIN_LEAVES_AFTER_KISSING_FAREWELL",
        "Zevran left after kissing the Warden goodbye",
        "Companions"
    ),
    da2_plot_bool!(
        2108,
        "ZEVRAN_MAIN_LEAVES_FOR_GOOD",
        "Zevran left for good",
        "Companions"
    ),
];
