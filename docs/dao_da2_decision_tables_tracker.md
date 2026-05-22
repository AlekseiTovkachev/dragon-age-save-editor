# DAO → DA2 Plot Flag Decision Tracker

This file is organized as a **decision tracker**.

Each section has one table. Each row is one possible option/outcome.

Columns:

| Column | Meaning |
|---|---|
| Option | User-facing decision option |
| Registers when | The condition that should make DA2 recognize the outcome |
| Set | Values to write |
| Clear contradictions | Other values in the same contradiction group that should be cleared |
| Notes | Prerequisites, derived logic, or uncertainty |

General rule:

```txt
Before setting an option, clear the other options in the same decision group.
```

Most values are boolean:

```txt
0 = false
1 = true
```

Important identity values:

```txt
1000 DAO_HERO_GENDER
  1 = male
  2 = female

1001 DAO_HERO_RACE
  1 = dwarf
  2 = elf
  3 = human
```

---

# 1. Broken Circle

Contradiction group:

```txt
2012 CIR_MAIN_MAGES_IN_ARMY
2013 CIR_MAIN_TEMPLARS_IN_ARMY
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Mages allied | `2012 = 1` | `2012 = 1` | `2013 = 0` | Recorded, but not a major DA2 content switch. |
| Templars allied | `2013 = 1` | `2013 = 1` | `2012 = 0` | Recorded, but not a major DA2 content switch. |

---

# 2. Nature of the Beast

Contradiction group:

```txt
2015 NTB_MAIN_ELVES_PROMISED_ALLIANCE
2016 NTB_MAIN_WEREWOLVES_PROMISED_ALLIANCE
2017 NTB_MAIN_ZATHRIAN_SACRIFICES_HIMSELF
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Dalish allied | `2015 = 1` | `2015 = 1` | `2016 = 0`, `2017 = 0` | DA2-relevant. Enables a Wounded Coast encounter. |
| Dalish allied + werewolves cured | `2017 = 1`; optionally `2015 = 1` | Recommended strict: `2017 = 1` | `2016 = 0`; optionally `2015 = 0` | The vault list treats “elves allied” and “werewolves cured” as separate flags. For contradiction-free logic, prefer only `2017 = 1`. If you want to represent the story more literally, allow `2015 = 1` + `2017 = 1`, but mark as compound. |
| Werewolves allied | `2016 = 1` | `2016 = 1` | `2015 = 0`, `2017 = 0` | DA2-relevant. Enables `Changing One's Nature`. |

---

# 3. Paragon of Her Kind: Anvil / Branka / Caridin

Contradiction group:

```txt
2081 ORZ_ANVIL_BRANKA_ALIVE
2082 ORZ_ANVIL_BRANKA_SUICIDES
2083 ORZ_ANVIL_COMPLETED_CARIDIN
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Branka allied, Anvil preserved | `2081 = 1` | `2081 = 1` | `2082 = 0`, `2083 = 0` | Branka lives. Anvil is not destroyed. |
| Branka allied, Anvil destroyed | `2082 = 1` | `2082 = 1` | `2081 = 0`, `2083 = 0` | Branka commits suicide. Counts as Anvil destroyed. |
| Caridin allied, Anvil destroyed | `2083 = 1` | `2083 = 1` | `2081 = 0`, `2082 = 0` | Counts as Anvil destroyed. |

Derived flag:

```txt
Anvil destroyed registers if:
  2082 = 1 OR 2083 = 1
```

---

# 4. Paragon of Her Kind: Orzammar Ruler

Contradiction group:

```txt
2018 ORZ_MAIN_KING_IS_BHELEN
2019 ORZ_MAIN_KING_IS_HARROWMONT
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Bhelen rules | `2018 = 1` | `2018 = 1` | `2019 = 0` | DA2-relevant. Enables `Last of His Line`. |
| Harrowmont rules | `2019 = 1` | `2019 = 1` | `2018 = 0` | Recorded, but no major DA2 quest result. |

---

# 5. Arl of Redcliffe: Village

Contradiction group:

```txt
2098 ARL_SIEGE_VILLAGE_DEFENDED
2099 ARL_SIEGE_VILLAGE_ABANDONED
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Village defended | `2098 = 1` | `2098 = 1` | `2099 = 0` | Recorded. |
| Village abandoned | `2099 = 1` | `2099 = 1` | `2098 = 0` | Recorded. |

---

# 6. Arl of Redcliffe: Connor / Isolde

Contradiction group:

```txt
2007 ARL_REMOVE_DEMON_CONNOR_ALIVE
2008 ARL_REMOVE_DEMON_CONNOR_KILLED
2086 ARL_REMOVE_DEMON_JOWAN_DOES_RITUAL
2087 ARL_REMOVE_DEMON_CIRCLE_DOES_RITUAL
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Connor saved, blood magic ritual, Isolde killed | `2007 = 1` and `2086 = 1` | `2007 = 1`, `2086 = 1` | `2008 = 0`, `2087 = 0` | Jowan/blood magic route. |
| Connor saved, Circle ritual, Isolde lives | `2007 = 1` and `2087 = 1` | `2007 = 1`, `2087 = 1` | `2008 = 0`, `2086 = 0` | Circle route. |
| Connor killed | `2008 = 1` | `2008 = 1` | `2007 = 0`, `2086 = 0`, `2087 = 0` | Do not leave a rescue ritual flag active. |

---

# 7. Urn of Sacred Ashes

Contradiction group:

```txt
2014 URN_ASHES_REVEALED_TO_WORLD
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Sacred Ashes revealed | `2014 = 1` | `2014 = 1` | None | DA2-relevant. Enables `Miracle Makers`. |
| Sacred Ashes remain unknown | `2014 = 0` | `2014 = 0` | None | No major DA2 result. |

---

# 8. Warden Gender

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Male Warden | `1000 = 1` | `1000 = 1` | None | Needed for Anora prince-consort. |
| Female Warden | `1000 = 2` | `1000 = 2` | None | Needed for Alistair queen. |

---

# 9. Warden Race

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Dwarf | `1001 = 1` | `1001 = 1` | None | Should match dwarf origin. |
| Elf | `1001 = 2` | `1001 = 2` | None | Should match elf origin. |
| Human | `1001 = 3` | `1001 = 3` | None | Needed for human noble political marriages. |

---

# 10. Warden Origin

Contradiction group:

```txt
2000 GEN_BACK_CIRCLE
2001 GEN_BACK_DWARF_COMMONER
2002 GEN_BACK_DWARF_NOBLE
2003 GEN_BACK_CITY
2004 GEN_BACK_ELF_DALISH
2005 GEN_BACK_HUMAN_NOBLE
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Circle mage | `2000 = 1` | `2000 = 1` | `2001 = 0`, `2002 = 0`, `2003 = 0`, `2004 = 0`, `2005 = 0` | Race should be human or elf. For human mage, also set `1001 = 3`. |
| Dwarf commoner | `2001 = 1` | `2001 = 1`, `1001 = 1` | Other origins = `0` | Dwarf race. |
| Dwarf noble | `2002 = 1` | `2002 = 1`, `1001 = 1` | Other origins = `0` | Dwarf race. |
| City elf | `2003 = 1` | `2003 = 1`, `1001 = 2` | Other origins = `0` | Elf race. |
| Dalish elf | `2004 = 1` | `2004 = 1`, `1001 = 2` | Other origins = `0` | Elf race. |
| Human noble | `2005 = 1` | `2005 = 1`, `1001 = 3` | Other origins = `0` | Required for Anora/Alistair political spouse outcomes. |

---

# 11. Landsmeet: Ruler Outcome

Main contradiction group:

```txt
2020 LANDSMEET_ALISTAIR_ENGAGED_TO_ANORA
2021 LANDSMEET_ALISTAIR_IS_SOLE_KING
2024 LANDSMEET_ANORA_ENGAGED_TO_PLAYER
2026 LANDSMEET_ALISTAIR_ENGAGED_TO_PLAYER
2027 LANDSMEET_ANORA_IS_SOLE_QUEEN
```

Related Alistair fate group:

```txt
2022 LANDSMEET_ALISTAIR_LEAVES_FOREVER
2023 LANDSMEET_ALISTAIR_KILLED
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Alistair + Anora rule together | `2020 = 1` and Alistair did not Ultimate Sacrifice | `2020 = 1` | `2021 = 0`, `2022 = 0`, `2023 = 0`, `2024 = 0`, `2026 = 0`, `2027 = 0` | Counts as Alistair king and Anora queen. |
| Alistair + Warden | `2026 = 1`, female human noble, Alistair alive, Warden alive | `2026 = 1`, `1000 = 2`, `1001 = 3`, `2005 = 1` | `2020 = 0`, `2021 = 0`, `2022 = 0`, `2023 = 0`, `2024 = 0`, `2027 = 0`; other origins = `0` | Warden is Alistair's queen. Must not kill Warden or Alistair in climax. |
| Anora + Warden | `2024 = 1`, male human noble, Warden alive | `2024 = 1`, `2027 = 1`, `1000 = 1`, `1001 = 3`, `2005 = 1` | `2020 = 0`, `2021 = 0`, `2022 = 0`, `2023 = 0`, `2026 = 0`; other origins = `0` | Warden is Anora's prince-consort, not literally king. |
| Alistair rules alone | `2021 = 1` and Alistair did not Ultimate Sacrifice | `2021 = 1` | `2020 = 0`, `2022 = 0`, `2023 = 0`, `2024 = 0`, `2026 = 0`, `2027 = 0` | DA2-relevant. Enables King Alistair. |
| Anora rules alone | `2027 = 1` and no Alistair king flags | `2027 = 1` | `2020 = 0`, `2021 = 0`, `2024 = 0`, `2026 = 0` | Then choose Alistair fate separately: exiled, Grey Warden, or executed. |

---

# 12. Landsmeet: If Alistair Does Not Rule

This applies mostly when Anora rules.

Contradiction group:

```txt
2022 LANDSMEET_ALISTAIR_LEAVES_FOREVER
2023 LANDSMEET_ALISTAIR_KILLED
Derived: Alistair remains Grey Warden if not king, not exiled, not executed, and did not Ultimate Sacrifice.
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Alistair exiled | `2022 = 1` | `2022 = 1` | `2020 = 0`, `2021 = 0`, `2023 = 0`, `2026 = 0` | DA2 drunk Alistair. Usually pair with `2027 = 1`. |
| Alistair remains Grey Warden | Alistair not king, `2022 = 0`, `2023 = 0`, and no Ultimate Sacrifice | `2027 = 1`, `2022 = 0`, `2023 = 0` | `2020 = 0`, `2021 = 0`, `2026 = 0` | There is no direct “Alistair Grey Warden” vault variable. It is derived. |
| Alistair executed | `2023 = 1` | `2023 = 1` | `2020 = 0`, `2021 = 0`, `2022 = 0`, `2026 = 0` | Usually pair with Loghain recruited/lives. |

---

# 13. Loghain

Contradiction group:

```txt
2025 LANDSMEET_LOGHAIN_KILLED
2097 LANDSMEET_LOGHAIN_LIVES
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Loghain recruited / lives | `2097 = 1` | `2097 = 1` | `2025 = 0` | If Loghain kills Archdemon and ritual is not performed, he is dead despite this. |
| Loghain executed / killed | `2025 = 1` | `2025 = 1` | `2097 = 0`, usually `2029 = 0` | Do not set Loghain as Archdemon killer if executed. |

---

# 14. Ritual

Important:

```txt
2104 RITUAL_PERFORMED
```

The ritual is not the same as the Archdemon killer. It decides whether the killer survives.

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Ritual performed with Alistair | `2104 = 1` and `2028 = 1` | `2104 = 1`, `2028 = 1` | `2029 = 0`, `2030 = 0` | Alistair survives killing Archdemon. Do not use if Alistair is executed/exiled unless intentionally weird. |
| Ritual performed with Warden | `2104 = 1` and `2030 = 1` | `2104 = 1`, `2030 = 1` | `2028 = 0`, `2029 = 0` | Safest default for most worldstates. |
| Ritual performed with Loghain | `2104 = 1` and `2029 = 1` | `2104 = 1`, `2029 = 1`, `2097 = 1` | `2028 = 0`, `2030 = 0`, `2025 = 0` | Loghain survives killing Archdemon. |
| No ritual | `2104 = 0` | `2104 = 0` | None | Whoever has the Archdemon killer flag dies. |

---

# 15. Archdemon Killer

Contradiction group:

```txt
2028 CLIMAX_ALISTAIR_KILLS_ARCHDEMON
2029 CLIMAX_LOGHAIN_KILLS_ARCHDEMON
2030 CLIMAX_PLAYER_KILLS_ARCHDEMON
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Alistair killed Archdemon | `2028 = 1` | `2028 = 1` | `2029 = 0`, `2030 = 0` | If `2104 = 0`, Alistair died. |
| Warden killed Archdemon | `2030 = 1` | `2030 = 1` | `2028 = 0`, `2029 = 0` | If `2104 = 0`, Warden died. |
| Loghain killed Archdemon | `2029 = 1` | `2029 = 1` | `2028 = 0`, `2030 = 0` | If `2104 = 0`, Loghain died. |

Recommended safe default:

```txt
2030 = 1
2104 = 1
```

This means Warden slew the Archdemon and survived.

---

# 16. Epilogue Boons

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Circle boon / Circle independence | `2032 = 1` | `2032 = 1` | None | Recorded. |
| Dalish boon / Dalish granted land | `2033 = 1` | `2033 = 1` | None | DA2-relevant. Merrill can mention it. |
| Warden chancellor | No supported flag found in checked DA2 vault list | Unknown | Unknown | Keep as unimplemented unless confirmed by save inspection. |

---

# 17. Alistair Romance

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Alistair romance active | `2040 = 1` | `2040 = 1` | None | Should usually require female Warden and Alistair alive. |
| Alistair romance not active | `2040 = 0` | `2040 = 0` | None | — |

---

# 18. Leliana

Contradiction group:

```txt
2038 GEN_LELIANA_RECRUITED
2042 APP_LELIANA_ROMANCE_ACTIVE
2044 LELIANA_MAIN_LEAVES_LOTHERING_FOREVER
2045 LELIANA_MAIN_ATTACKS_PC
```

Important:

```txt
2038 means Leliana recruited AND stayed until the end.
It should be false if Leliana left or was killed.
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Recruited and stayed | `2038 = 1` | `2038 = 1` | `2044 = 0`, `2045 = 0` | Use for alive/stayed Leliana. |
| Not recruited | `2044 = 1` | `2044 = 1` | `2038 = 0`, `2042 = 0`, `2045 = 0` | The supported flag is “Leliana not recruited.” |
| Left | `2044 = 1` | `2044 = 1` | `2038 = 0`, `2042 = 0`, `2045 = 0` | Same vault flag appears to represent not recruited / left Lothering. |
| Killed | `2045 = 1` | `2045 = 1` | `2038 = 0`, `2042 = 0`, `2044 = 0` | Do not also set recruited/stayed. |
| Stayed | `2038 = 1` | `2038 = 1` | `2044 = 0`, `2045 = 0` | Same as recruited/stayed. |
| Romance active | `2042 = 1` and `2038 = 1` | `2042 = 1`, `2038 = 1` | `2044 = 0`, `2045 = 0` | Romance should require Leliana stayed. |
| Romance not active | `2042 = 0` | `2042 = 0` | None | — |

---

# 19. Zevran

Contradiction group:

```txt
2039 GEN_ZEVRAN_RECRUITED
2048 APP_ZEVRAN_ROMANCE_ACTIVE
2050 ZEVRAN_MAIN_GOES_HOSTILE
2051 ZEVRAN_MAIN_LEAVES_PARTY_AND_GOES
2105 ZEVRAN_MAIN_START_AMBUSH_FIGHT_ZEVRAN_ENEMY
2106 ZEVRAN_MAIN_KILLED_BEFORE_INTRODUCTION
2107 ZEVRAN_MAIN_LEAVES_AFTER_KISSING_FAREWELL
2108 ZEVRAN_MAIN_LEAVES_FOR_GOOD
```

Important:

```txt
2039 means Zevran recruited AND stayed until the end.
It should be false if Zevran died, became hostile, or left.
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Recruited at first encounter / stayed | `2039 = 1` | `2039 = 1` | `2050 = 0`, `2051 = 0`, `2105 = 0`, `2106 = 0`, `2107 = 0`, `2108 = 0` | DA2-relevant. Enables Zevran quest. |
| Sent away at first encounter | `2051 = 1` or similar left flag | `2039 = 0`, `2051 = 1` | `2048 = 0`, `2050 = 0`, `2105 = 0`, `2106 = 0`, `2107 = 0`, `2108 = 0` | Use left-party representation. |
| Killed at first encounter | `2106 = 1` | `2039 = 0`, `2106 = 1` | `2048 = 0`, `2050 = 0`, `2051 = 0`, `2105 = 0`, `2107 = 0`, `2108 = 0` | Do not set recruited/stayed. |
| Left later | `2051 = 1` or `2107 = 1` or `2108 = 1` | `2039 = 0`, choose one left flag | `2048 = 0`, kill/hostile flags = `0` | Pick one left reason; do not set all. |
| Killed later / hostile | `2050 = 1` or `2105 = 1` | `2039 = 0`, choose one kill/hostile flag | `2048 = 0`, left flags = `0` | Pick one kill reason; do not set all. |
| Stayed | `2039 = 1` | `2039 = 1` | All dead/left/hostile flags = `0` | Same as recruited/stayed. |
| Romance active | `2048 = 1` and `2039 = 1` | `2048 = 1`, `2039 = 1` | Dead/left/hostile flags = `0` | Romance should require Zevran stayed. |
| Romance not active | `2048 = 0` | `2048 = 0` | None | — |

---

# 20. Isabela Sex

Contradiction/compound group:

```txt
2052 ISABELA_AND_ALISTAIR_THREESOME
2053 ISABELA_AND_LELIANA_THREESOME
2054 ISABELA_AND_ZEVRAN_THREESOME
2055 ISABELA_IN_FOURSOME
2056 ISABELA_SLEPT_WITH
```

Important:

```txt
2056 means Isabela slept with the Warden.
2052/2053/2054 are threesome flags.
2055 is foursome.
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Threesome with Leliana | `2053 = 1` | `2053 = 1`, `2038 = 1` | `2052 = 0`, `2054 = 0`, `2055 = 0`; optionally `2056 = 0` | Leliana should be recruited/stayed. |
| Threesome with Alistair | `2052 = 1` | `2052 = 1` | `2053 = 0`, `2054 = 0`, `2055 = 0`; optionally `2056 = 0` | Alistair should be alive. |
| Threesome with Zevran | `2054 = 1` | `2054 = 1`, `2039 = 1` | `2052 = 0`, `2053 = 0`, `2055 = 0`; optionally `2056 = 0` | Zevran should be recruited/stayed. |
| Foursome with Zevran and Leliana | `2055 = 1` | `2055 = 1`, `2038 = 1`, `2039 = 1` | `2052 = 0`, `2053 = 0`, `2054 = 0`; optionally `2056 = 0` | Leliana and Zevran should both be recruited/stayed. |
| Only with Warden | `2056 = 1` | `2056 = 1` | `2052 = 0`, `2053 = 0`, `2054 = 0`, `2055 = 0` | Cleanest simple Isabela encounter. |
| None | all Isabela flags `0` | `2052 = 0`, `2053 = 0`, `2054 = 0`, `2055 = 0`, `2056 = 0` | None | — |

---

# 21. Awakening: Orlesian Warden-Commander

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Orlesian Warden-Commander | `2057 = 1` | `2057 = 1` | None | Use for Awakening-only Orlesian commander. |
| Not Orlesian / Hero of Ferelden continued | `2057 = 0` | `2057 = 0` | None | Default for surviving DAO Warden import. |

---

# 22. Awakening: Anders

Contradiction group:

```txt
2064 GXA_ANDERS_RECRUITED
2066 COD_CHA_ANDERS_DIED_IN_VGK_SIEGE
```

Important exception:

```txt
The source notes that Anders/Nathaniel death at Vigil's Keep is marked in the epilogue rather than with the normal follower-recruited rule.
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Anders recruited | `2064 = 1` | `2064 = 1` | None | If he died at Vigil's Keep, `2066` may also mark that epilogue death. |
| Anders not recruited | `2064 = 0` | `2064 = 0` | `2066 = 0` | — |
| Anders left | No supported specific flag found | Unknown | Unknown | Keep unimplemented unless confirmed by save inspection. |
| Anders died | `2066 = 1` | `2066 = 1` | Usually `2064 = 1` may remain possible | Death at Vigil's Keep. Unlike normal followers, recruited may still be true. |
| Anders stayed | `2064 = 1` and `2066 = 0` | `2064 = 1`, `2066 = 0` | None | Clean alive/stayed state. |

---

# 23. Awakening: Nathaniel

Contradiction group:

```txt
2065 GXA_NATHANIEL_RECRUITED
2084 COD_CHA_NATHANIEL_DIED_IN_VGK_SIEGE
```

Important exception:

```txt
Nathaniel death at Vigil's Keep is marked in the epilogue rather than with the normal follower-recruited rule.
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Nathaniel recruited | `2065 = 1` | `2065 = 1` | None | DA2-relevant if he stays alive. |
| Nathaniel not recruited | `2065 = 0` | `2065 = 0` | `2084 = 0` | — |
| Nathaniel left | No supported specific flag found | Unknown | Unknown | Keep unimplemented unless confirmed by save inspection. |
| Nathaniel killed / died at Vigil's Keep | `2084 = 1` | `2084 = 1` | Usually keep or clear `2065` depending on interpretation | Source exception says epilogue death is separate. For simple dead/unavailable, use `2065 = 0`, `2084 = 1`. |
| Nathaniel stayed / alive | `2065 = 1` and `2084 = 0` | `2065 = 1`, `2084 = 0` | None | DA2-relevant. Enables `Finding Nathaniel`, though import can be fragile. |

---

# 24. Awakening: Amaranthine Defenses

These are named like Amaranthine support choices in the user's list, but the supported vault names are:

```txt
2060 VGK_DEFENSES_ROADS_CHOSEN
2061 VGK_DEFENSES_FARMS_CHOSEN
2062 VGK_HERREN_COMPLETED_SILVERITE
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Roads defended | `2060 = 1` | `2060 = 1` | Optional: `2061 = 0` if strict single-choice mode | Listed as Amaranthine roads defended. |
| Farms defended | `2061 = 1` | `2061 = 1` | Optional: `2060 = 0` if strict single-choice mode | Listed as Amaranthine farms defended. |
| Both roads and farms | `2060 = 1` and `2061 = 1` | `2060 = 1`, `2061 = 1` | None | Allow in maximum-content mode if desired. |
| Silver Order formed | `2062 = 1` | `2062 = 1` | None | Listed as Silver Order formed / silverite completed. |
| Silver Order not formed | `2062 = 0` | `2062 = 0` | None | — |

---

# 25. Awakening: Amaranthine vs Vigil's Keep

Contradiction / maximum-content group:

```txt
2058 VGK_SIEGE_COMPLETED
2059 AOA_SIEGE_AMARANTHINE_SAVED
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Amaranthine defended | `2059 = 1` | `2059 = 1` | Strict mode: `2058 = 0` | DA2-relevant. Enables `Secret Rendezvous`. |
| Keep defended | `2058 = 1` | `2058 = 1`; recommended also `2060 = 1`, `2061 = 1`, `2062 = 1` | Strict mode: `2059 = 0` | DA2-relevant. Enables `The Conspirators`. |
| Both Amaranthine and Keep defended | `2058 = 1` and `2059 = 1` | `2058 = 1`, `2059 = 1`, `2060 = 1`, `2061 = 1`, `2062 = 1` | None | Maximum DA2-content mode. May be less lore-strict. |

---

# 26. Awakening: Architect

Contradiction group:

```txt
2063 LTM_MAIN_KILLED_ARCHITECT
2096 LTM_MAIN_DEAL_WITH_ARCHITECT
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Architect spared | `2096 = 1` | `2096 = 1` | `2063 = 0` | DA2-relevant. Architect's Legacy and Nathaniel mention. |
| Architect killed | `2063 = 1` | `2063 = 1` | `2096 = 0` | Recorded. |

---

# 27. Warden's Keep

Main flag:

```txt
2094 GWB_STARTED
```

Dependent flags:

```txt
2067 GWB_MAIN_SOPHIA_KILLED
2068 GWB_MAIN_AVERNUS_KILLED
2070 GWB_AVERNUS_DOING_BAD_EXPERIMENTS
2071 GWB_AVERNUS_DOING_GOOD_EXPERIMENTS
```

Important inverse logic:

```txt
Sophia alive registers when:
  2094 = 1 and 2067 = 0

Avernus alive registers when:
  2094 = 1 and 2068 = 0
```

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Warden's Keep not completed / not started | `2094 = 0` | `2094 = 0` | Optional: `2067 = 0`, `2068 = 0`, `2070 = 0`, `2071 = 0` | If not started, lower flags are irrelevant. |
| Warden's Keep completed / started | `2094 = 1` | `2094 = 1` | None | Required for Sophia/Avernus alive flags to matter. |
| Sophia alive, Avernus killed | `2094 = 1`, `2067 = 0`, `2068 = 1` | `2094 = 1`, `2067 = 0`, `2068 = 1` | `2070 = 0`, `2071 = 0` | DA2-relevant. Enables `Terror on the Coast`. |
| Sophia killed, Avernus alive | `2094 = 1`, `2067 = 1`, `2068 = 0` | `2094 = 1`, `2067 = 1`, `2068 = 0` | choose one research flag | DA2-relevant. Enables `Dark Epiphany`. |
| Both Sophia and Avernus killed | `2094 = 1`, `2067 = 1`, `2068 = 1` | `2094 = 1`, `2067 = 1`, `2068 = 1` | `2070 = 0`, `2071 = 0` | Clean no-survivor state. |
| Avernus ethical research | `2071 = 1` | `2071 = 1` | `2070 = 0` | Only meaningful if Avernus alive. |
| Avernus evil research | `2070 = 1` | `2070 = 1` | `2071 = 0` | Only meaningful if Avernus alive. |

---

# 28. Shale

The checked DA2 supported vault flag article does **not** provide clean Shale recruited/killed/left vault IDs.

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Shale recruited | Unknown | Unknown | Unknown | Do not implement unless confirmed by save inspection or another trusted flag list. |
| Shale killed | Unknown | Unknown | Unknown | Do not invent a flag. |
| Shale left | Unknown | Unknown | Unknown | Do not invent a flag. |

Implementation recommendation:

```txt
Keep Shale hidden or mark as unsupported/unconfirmed.
```

---

# 29. Cailan's Body / Return to Ostagar

The checked DA2 supported vault flag article does **not** list Cailan body flags.

| Option | Registers when | Set | Clear contradictions | Notes |
|---|---:|---|---|---|
| Cailan's body burned | Unknown | Unknown | Unknown | Not supported in checked DA2 import-vault flag list. |
| Cailan's body left | Unknown | Unknown | Unknown | Not supported in checked DA2 import-vault flag list. |
| Cailan's body left to wolves | Unknown | Unknown | Unknown | Not supported in checked DA2 import-vault flag list. |

Implementation recommendation:

```txt
Keep Return to Ostagar body choices out of the main editor unless another confirmed source is added.
```

---

# Cross-Decision Validation Rules

Use these rules to track contradictions.

## Identity

Warn if more than one origin is true:

```txt
2000 + 2001 + 2002 + 2003 + 2004 + 2005 > 1
```

Warn if human noble political marriage does not match identity:

```txt
2026 = 1 requires:
  1000 = 2
  1001 = 3
  2005 = 1

2024 = 1 requires:
  1000 = 1
  1001 = 3
  2005 = 1
```

## Landsmeet

Warn if:

```txt
Alistair king and exiled:
  (2020 = 1 OR 2021 = 1 OR 2026 = 1)
  AND 2022 = 1

Alistair king and executed:
  (2020 = 1 OR 2021 = 1 OR 2026 = 1)
  AND 2023 = 1

Alistair exiled and executed:
  2022 = 1 AND 2023 = 1

Loghain killed and alive:
  2025 = 1 AND 2097 = 1

More than one Archdemon killer:
  count(2028, 2029, 2030) > 1

No Archdemon killer:
  count(2028, 2029, 2030) = 0
```

Ultimate Sacrifice validation:

```txt
If 2104 = 0:
  2028 = 1 means Alistair is dead.
  2029 = 1 means Loghain is dead.
  2030 = 1 means Warden is dead.

If 2104 = 1:
  The Archdemon killer survives.
```

## Companions

Warn if:

```txt
Leliana stayed and killed:
  2038 = 1 AND 2045 = 1

Leliana stayed and not recruited/left:
  2038 = 1 AND 2044 = 1

Leliana romance active but not stayed:
  2042 = 1 AND 2038 = 0

Zevran stayed and dead/left/hostile:
  2039 = 1 AND any of 2050, 2051, 2105, 2106, 2107, 2108 = 1

Zevran romance active but not stayed:
  2048 = 1 AND 2039 = 0
```

## Isabela

Warn if:

```txt
Isabela + Leliana threesome/foursome but Leliana did not stay:
  (2053 = 1 OR 2055 = 1)
  AND 2038 = 0

Isabela + Zevran threesome/foursome but Zevran did not stay:
  (2054 = 1 OR 2055 = 1)
  AND 2039 = 0
```

## Warden's Keep

Warn if:

```txt
Avernus research both ethical and evil:
  2070 = 1 AND 2071 = 1

Avernus research set while Avernus killed:
  2068 = 1 AND (2070 = 1 OR 2071 = 1)

Sophia/Avernus alive logic used while Warden's Keep not started:
  2094 = 0 AND meaningful Warden's Keep flags are set
```

## Awakening

Warn if:

```txt
Architect killed and spared:
  2063 = 1 AND 2096 = 1

Nathaniel recruited/stayed and died:
  2065 = 1 AND 2084 = 1

Anders recruited/stayed and died:
  2064 = 1 AND 2066 = 1
```

---

# Suggested UI Implementation

For each decision group:

```txt
1. Show the table options as radio buttons if mutually exclusive.
2. Show checkboxes only for independent boons/toggles.
3. When the user selects an option:
   - apply Set
   - apply Clear contradictions
   - run validation
4. Show warnings near the affected section.
```

Recommended radio groups:

```txt
Broken Circle
Nature of the Beast
Orzammar ruler
Anvil outcome
Redcliffe village
Connor outcome
Warden gender
Warden race
Warden origin
Landsmeet ruler
Alistair fate if not ruler
Loghain fate
Archdemon killer
Leliana state
Zevran state
Isabela encounter
Architect fate
Warden's Keep state
```

Recommended checkboxes/toggles:

```txt
Sacred Ashes revealed
Circle boon
Dalish boon
Alistair romance active
Leliana romance active
Zevran romance active
Orlesian Warden-Commander
Roads defended
Farms defended
Silver Order formed
Amaranthine defended
Vigil's Keep defended
Avernus research ethical/evil, but only if Avernus alive
