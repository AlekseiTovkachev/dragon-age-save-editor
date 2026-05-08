// Mock data for the hi-fi prototype

window.PARTY = [
  { key:"main", name:"Solona Amell", role:"Mage / Warden", lvl:14, dirty:true,
    xp:152400, approval:null,
    stats:{ Strength:22, Dexterity:18, Willpower:35, Magic:42, Cunning:24, Constitution:26 },
    pools:{ Attribute:0, Skill:1, Talent:0, Specialization:1 } },
  { key:"alistair", name:"Alistair", role:"Warrior / Templar", lvl:13, dirty:false,
    xp:138200, approval:78,
    stats:{ Strength:36, Dexterity:24, Willpower:22, Magic:18, Cunning:18, Constitution:32 },
    pools:{ Attribute:0, Skill:0, Talent:1, Specialization:0 } },
  { key:"morrigan", name:"Morrigan", role:"Apostate Mage", lvl:13, dirty:false,
    xp:138200, approval:42,
    stats:{ Strength:18, Dexterity:20, Willpower:30, Magic:40, Cunning:28, Constitution:22 },
    pools:{ Attribute:0, Skill:0, Talent:0, Specialization:1 } },
  { key:"leliana", name:"Leliana", role:"Bard / Rogue", lvl:12, dirty:false,
    xp:122400, approval:65,
    stats:{ Strength:18, Dexterity:34, Willpower:22, Magic:18, Cunning:30, Constitution:22 },
    pools:{ Attribute:1, Skill:0, Talent:0, Specialization:0 } },
  { key:"sten", name:"Sten", role:"Qunari Beresaad", lvl:11, dirty:false,
    xp:108300, approval:30,
    stats:{ Strength:38, Dexterity:22, Willpower:24, Magic:14, Cunning:16, Constitution:34 },
    pools:{ Attribute:0, Skill:0, Talent:0, Specialization:0 } },
  { key:"wynne", name:"Wynne", role:"Senior Enchanter", lvl:12, dirty:false,
    xp:122400, approval:55,
    stats:{ Strength:18, Dexterity:18, Willpower:36, Magic:38, Cunning:24, Constitution:24 },
    pools:{ Attribute:0, Skill:0, Talent:0, Specialization:1 } },
  { key:"zevran", name:"Zevran", role:"Antivan Crow", lvl:12, dirty:false,
    xp:122400, approval:50,
    stats:{ Strength:22, Dexterity:36, Willpower:20, Magic:14, Cunning:30, Constitution:24 },
    pools:{ Attribute:0, Skill:0, Talent:0, Specialization:0 } },
  { key:"dog", name:"Dog", role:"Mabari War Hound", lvl:10, dirty:false,
    xp:92100, approval:80,
    stats:{ Strength:32, Dexterity:24, Willpower:20, Magic:0, Cunning:14, Constitution:30 },
    pools:{ Attribute:0, Skill:0, Talent:0, Specialization:0 } },
];

// Ability trees per kind
window.ABILITY_TREES = {
  skills: [
    { id:"combat", name:"Combat Training", ranks:[
      { name:"Combat Training", desc:"+1 weapon damage class. Required for warrior weapon talents.", owned:true, locked:true },
      { name:"Improved Combat Training", desc:"Further damage class bonus and improved threat generation.", owned:true, locked:false },
      { name:"Expert Combat Training", desc:"Allows specialization in two-handed weapons.", owned:false },
      { name:"Master Combat Training", desc:"Unlocks the highest tier combat techniques.", owned:false },
    ]},
    { id:"coercion", name:"Coercion", ranks:[
      { name:"Coercion", desc:"Unlocks persuade and intimidate dialogue options.", owned:true },
      { name:"Improved Coercion", desc:"More dialogue options and influence over companions.", owned:false },
      { name:"Expert Coercion", desc:"Hardest dialogue checks become available.", owned:false },
      { name:"Master Coercion", desc:"Always succeed at low-tier persuade and intimidate.", owned:false },
    ]},
    { id:"stealing", name:"Stealing", ranks:[
      { name:"Stealing", desc:"Pickpocket non-hostile NPCs for coin.", owned:false },
      { name:"Improved Stealing", desc:"Steal common items and increased coin yield.", owned:false },
      { name:"Expert Stealing", desc:"Steal uncommon items.", owned:false },
      { name:"Master Stealing", desc:"Steal even rare and unique items.", owned:false },
    ]},
    { id:"trap", name:"Trap-Making", ranks:[
      { name:"Trap-Making", desc:"Craft basic traps from kits and reagents.", owned:false },
      { name:"Improved Trap-Making", desc:"Craft larger and better traps.", owned:false },
      { name:"Expert Trap-Making", desc:"Craft elemental and timed traps.", owned:false },
      { name:"Master Trap-Making", desc:"Craft the deadliest known traps.", owned:false },
    ]},
    { id:"survival", name:"Survival", ranks:[
      { name:"Survival", desc:"Bonus to natural resistances. Detect nearby enemies.", owned:true },
      { name:"Improved Survival", desc:"Larger detection radius, see hidden enemies.", owned:false },
      { name:"Expert Survival", desc:"Bonus party-wide nature damage resistance.", owned:false },
      { name:"Master Survival", desc:"Maximum nature resistance plus passive party health regen.", owned:false },
    ]},
    { id:"herb", name:"Herbalism", ranks:[
      { name:"Herbalism", desc:"Craft basic poultices and salves.", owned:false },
      { name:"Improved Herbalism", desc:"Craft greater and lyrium potions.", owned:false },
      { name:"Expert Herbalism", desc:"Craft potent and elemental potions.", owned:false },
      { name:"Master Herbalism", desc:"Craft the rarest, most powerful potions known.", owned:false },
    ]},
    { id:"poison", name:"Poison-Making", ranks:[
      { name:"Poison-Making", desc:"Craft basic poisons and salves.", owned:false },
      { name:"Improved Poison-Making", desc:"Craft acidic and venomous coatings.", owned:false },
      { name:"Expert Poison-Making", desc:"Craft the deadliest of poisons.", owned:false },
      { name:"Master Poison-Making", desc:"Craft Crow Poison and other rare toxins.", owned:false },
    ]},
    { id:"runes", name:"Runecrafting", ranks:[
      { name:"Runecrafting", desc:"Etch tier 1\u20132 runes onto socketed weapons.", owned:false },
      { name:"Improved Runecrafting", desc:"Etch up to tier 4 runes.", owned:false },
      { name:"Expert Runecrafting", desc:"Etch up to tier 6 runes.", owned:false },
      { name:"Master Runecrafting", desc:"Etch grandmaster runes.", owned:false },
    ]},
  ],
  talents: [
    { id:"weap_2h", name:"Two-Handed", ranks:[
      { name:"Powerful", desc:"+5 damage with two-handed weapons.", owned:false },
      { name:"Pommel Strike", desc:"Stuns target briefly.", owned:false },
      { name:"Indomitable", desc:"Immune to knockdown for 30 seconds.", owned:false },
      { name:"Sunder Arms / Armor", desc:"Reduces target attack/armor.", owned:false },
    ]},
    { id:"weap_dw", name:"Dual Weapon", ranks:[
      { name:"Dual-Weapon Training", desc:"Reduces dual-weapon penalty.", owned:false },
      { name:"Dual-Weapon Finesse", desc:"Off-hand weapon adds full attack damage.", owned:false },
      { name:"Dual-Weapon Expert", desc:"Bonus to attack with two weapons.", owned:false },
      { name:"Dual-Weapon Master", desc:"Largest dual-weapon attack speed bonus.", owned:false },
    ]},
    { id:"weap_arch", name:"Archery", ranks:[
      { name:"Melee Archer", desc:"Fire bows in melee without penalty.", owned:false },
      { name:"Aim", desc:"Trade attack speed for accuracy.", owned:false },
      { name:"Crippling Shot", desc:"Slows target movement.", owned:false },
      { name:"Arrow of Slaying", desc:"Massive damage on a single shot.", owned:false },
    ]},
  ],
  spells: [
    { id:"primal", name:"Primal", ranks:[
      { name:"Flame Blast", desc:"Cone of fire damage.", owned:true },
      { name:"Flaming Weapons", desc:"Add fire damage to party weapons.", owned:true },
      { name:"Fireball", desc:"Area-of-effect fire damage.", owned:true },
      { name:"Inferno", desc:"Persistent fire field.", owned:true, locked:true },
    ]},
    { id:"creation", name:"Creation", ranks:[
      { name:"Heal", desc:"Restores health to a single ally.", owned:true },
      { name:"Rejuvenate", desc:"Restores stamina or mana.", owned:false },
      { name:"Regeneration", desc:"Slow health regen on a single target.", owned:false },
      { name:"Mass Rejuvenation", desc:"Restores stamina/mana to all allies.", owned:false },
    ]},
    { id:"spirit", name:"Spirit", ranks:[
      { name:"Mind Blast", desc:"Stun nearby enemies briefly.", owned:false },
      { name:"Force Field", desc:"Renders ally invulnerable but immobile.", owned:false },
      { name:"Telekinetic Weapons", desc:"Adds spirit damage to party weapons.", owned:false },
      { name:"Crushing Prison", desc:"Crushes enemy with telekinetic force.", owned:false },
    ]},
    { id:"entropy", name:"Entropy", ranks:[
      { name:"Weakness", desc:"Lowers target's attack and damage.", owned:false },
      { name:"Paralyze", desc:"Holds target motionless.", owned:false },
      { name:"Misdirection Hex", desc:"Reduces target's missile chance.", owned:false },
      { name:"Mass Paralysis", desc:"Paralyzes all enemies in an area.", owned:false },
    ]},
    { id:"arcane", name:"Arcane", ranks:[
      { name:"Arcane Bolt", desc:"Single-target arcane projectile.", owned:false },
      { name:"Arcane Shield", desc:"Personal magical defense.", owned:false },
      { name:"Arcane Mastery", desc:"Bonus to spell power and resistance.", owned:false },
      { name:"Arcane Field", desc:"Damages enemies and dispels magic.", owned:false },
    ]},
  ],
};

window.ITEMS = [
  { name:"Starfang", cat:"Longsword", tier:7, stack:1, cost:450, mat:"Veridium", lvl:14, resref:"gen_im_wep_swd_lng_blk", props:[{n:"Damage bonus",p:"+5"},{n:"Attack bonus",p:"+3"},{n:"Critical chance",p:"+2.5%"}] },
  { name:"Vigilance", cat:"Greatsword", tier:6, stack:1, cost:380, mat:"Silverite", lvl:12, resref:"gen_im_wep_swd_2h_vig", props:[{n:"Damage bonus",p:"+4"},{n:"Armor penetration",p:"+2"}] },
  { name:"Andruil's Blessing", cat:"Light armor", tier:5, stack:1, cost:420, mat:"Drakeskin", lvl:11, resref:"gen_im_arm_lgt_drak", props:[{n:"Armor",p:"+12"},{n:"Dexterity",p:"+2"}] },
  { name:"Cailan's Shield", cat:"Shield", tier:5, stack:1, cost:310, mat:"Steel", lvl:10, resref:"gen_im_shi_caila", props:[{n:"Defense",p:"+8"}] },
  { name:"Lyrium Potion", cat:"Consumable", tier:2, stack:12, cost:40, mat:"\u2014", lvl:0, resref:"gen_im_pot_lyr_lgt", props:[{n:"Restore mana",p:"+50"}] },
  { name:"Health Poultice", cat:"Consumable", tier:1, stack:32, cost:15, mat:"\u2014", lvl:0, resref:"gen_im_pot_hlt_lgt", props:[{n:"Restore health",p:"+30"}] },
  { name:"Lifestone", cat:"Misc", tier:4, stack:2, cost:75, mat:"\u2014", lvl:0, resref:"gen_im_rea_lifest", props:[] },
  { name:"Volcanic Aurum", cat:"Misc", tier:6, stack:1, cost:180, mat:"\u2014", lvl:0, resref:"gen_im_rea_volau", props:[] },
  { name:"Elfroot", cat:"Misc", tier:1, stack:5, cost:8, mat:"\u2014", lvl:0, resref:"gen_im_rea_elfro", props:[] },
  { name:"Spider Ichor", cat:"Misc", tier:2, stack:3, cost:12, mat:"\u2014", lvl:0, resref:"gen_im_rea_spid", props:[] },
  { name:"Topsider's Honor", cat:"Greatsword", tier:6, stack:1, cost:340, mat:"Steel", lvl:11, resref:"gen_im_wep_2h_topsd", props:[{n:"Damage bonus",p:"+3"},{n:"Attack vs darkspawn",p:"+10%"}] },
  { name:"The Reaper's Vestments", cat:"Light armor", tier:6, stack:1, cost:520, mat:"Drakeskin", lvl:13, resref:"gen_im_arm_lgt_reap", props:[{n:"Armor",p:"+14"},{n:"Magic",p:"+3"}] },
  { name:"Dalish Bow", cat:"Bow", tier:4, stack:1, cost:260, mat:"Ironbark", lvl:8, resref:"gen_im_wep_bow_dal", props:[{n:"Damage bonus",p:"+2"}] },
  { name:"Lifegiver", cat:"Staff", tier:7, stack:1, cost:540, mat:"Sylvanwood", lvl:14, resref:"gen_im_wep_stf_life", props:[{n:"Spellpower",p:"+5"},{n:"Healing bonus",p:"+10%"}] },
  { name:"Backpack", cat:"Misc", tier:1, stack:1, cost:5, mat:"\u2014", lvl:0, resref:"gen_im_misc_bckp", props:[] },
  { name:"Tome of Skill", cat:"Consumable", tier:3, stack:2, cost:140, mat:"\u2014", lvl:0, resref:"gen_im_tome_skill", props:[{n:"Skill point",p:"+1"}] },
];

window.ITEM_CATEGORIES = ["All", "Weapons", "Armor", "Consumable", "Misc"];

window.PLOT_DECISIONS = [
  { era:"Origin", id:1101, q:"Hero of Ferelden's origin",
    opts:["Mage (Magi)", "Human Noble", "Dwarf Commoner", "Dwarf Noble", "Dalish Elf", "City Elf"], picked:0, modified:false },
  { era:"Origin", id:1102, q:"Hero of Ferelden's gender", opts:["Female", "Male"], picked:0, modified:false },
  { era:"Act 1", id:1204, q:"How was Loghain handled at the Landsmeet?", opts:["Executed by the Warden", "Spared and conscripted", "Executed by Alistair"], picked:0, modified:true },
  { era:"Act 1", id:1207, q:"Who took the throne of Ferelden?", opts:["Alistair as sole king", "Anora as sole queen", "Alistair & Anora (married)"], picked:2, modified:true },
  { era:"Companions", id:1311, q:"Alistair's fate after the Blight", opts:["Crowned king", "Made prince-consort", "A drunk in Kirkwall", "Dead at the archdemon"], picked:1, modified:false },
  { era:"Companions", id:1318, q:"Did Morrigan perform the Dark Ritual?", opts:["Yes \u2014 Warden survived", "Yes \u2014 Alistair did it", "Yes \u2014 Loghain did it", "No \u2014 Warden died"], picked:0, modified:true },
  { era:"Companions", id:1322, q:"Leliana's fate", opts:["Returned to the Chantry", "Went into hiding", "Stayed with the Wardens", "Killed during quest"], picked:0, modified:false },
  { era:"Companions", id:1325, q:"Zevran's fate", opts:["Romanced and travelling", "Sent away alive", "Killed during recruit"], picked:1, modified:false },
  { era:"World", id:1402, q:"Status of the Circle of Magi", opts:["Saved (allied)", "Annulled (templars sided)", "Mages won independence"], picked:0, modified:true },
  { era:"World", id:1404, q:"Fate of the Dalish elves at Brecilian", opts:["Werewolves cured \u2014 elves saved", "Elves slain \u2014 sided with werewolves", "Werewolves killed \u2014 curse lingers"], picked:0, modified:false },
  { era:"World", id:1408, q:"Who rules Orzammar?", opts:["Bhelen Aeducan", "Pyral Harrowmont", "Anarchy (no king)"], picked:0, modified:true },
  { era:"World", id:1412, q:"Anvil of the Void", opts:["Destroyed", "Preserved for Orzammar"], picked:0, modified:false },
  { era:"World", id:1418, q:"Urn of Sacred Ashes", opts:["Preserved", "Defiled with Andraste's blood"], picked:0, modified:false },
  { era:"Origins DLC", id:1601, q:"Did the Warden survive the Blight?", opts:["Yes", "No \u2014 sacrificed at Denerim"], picked:0, modified:false },
  { era:"Origins DLC", id:1604, q:"Awakening completed?", opts:["Yes \u2014 Vigil's Keep saved", "Yes \u2014 Amaranthine saved", "No / not played"], picked:0, modified:false },
];

window.PLOT_ERAS = ["All", "Origin", "Act 1", "Companions", "World", "Origins DLC"];
