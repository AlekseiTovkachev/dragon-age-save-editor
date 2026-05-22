use std::collections::BTreeMap;

struct ImplicationRule {
    trigger_id: u16,
    set_booleans: &'static [(u16, bool)],
    set_integers: &'static [(u16, i32)],
    clear_origin_group: bool,
}

const ORIGIN_GROUP: &[u16] = &[2000, 2001, 2002, 2003, 2004, 2005];

const IMPLICATIONS: &[ImplicationRule] = &[
    // 2026: Alistair+Warden wed → female human noble
    ImplicationRule {
        trigger_id: 2026,
        set_booleans: &[(2005, true)],
        set_integers: &[(1000, 2), (1001, 3)],
        clear_origin_group: true,
    },
    // 2024: Anora+Warden wed → male human noble
    ImplicationRule {
        trigger_id: 2024,
        set_booleans: &[(2005, true)],
        set_integers: &[(1000, 1), (1001, 3)],
        clear_origin_group: true,
    },
    // Isabela encounters → required companion recruited
    ImplicationRule {
        trigger_id: 2053,
        set_booleans: &[(2038, true)],
        set_integers: &[],
        clear_origin_group: false,
    },
    ImplicationRule {
        trigger_id: 2054,
        set_booleans: &[(2039, true)],
        set_integers: &[],
        clear_origin_group: false,
    },
    ImplicationRule {
        trigger_id: 2055,
        set_booleans: &[(2038, true), (2039, true)],
        set_integers: &[],
        clear_origin_group: false,
    },
    // Romance → companion recruited
    ImplicationRule {
        trigger_id: 2042,
        set_booleans: &[(2038, true)],
        set_integers: &[],
        clear_origin_group: false,
    },
    ImplicationRule {
        trigger_id: 2048,
        set_booleans: &[(2039, true)],
        set_integers: &[],
        clear_origin_group: false,
    },
];

pub fn apply_implications(booleans: &mut BTreeMap<u16, bool>, integers: &mut BTreeMap<u16, i32>) {
    for rule in IMPLICATIONS {
        if booleans.get(&rule.trigger_id).copied().unwrap_or(false) {
            for &(id, value) in rule.set_booleans {
                booleans.insert(id, value);
            }
            for &(id, value) in rule.set_integers {
                integers.insert(id, value);
            }
            if rule.clear_origin_group {
                for &origin_id in ORIGIN_GROUP {
                    // Only clear if not explicitly set to true by this rule
                    if !rule
                        .set_booleans
                        .iter()
                        .any(|&(id, v)| id == origin_id && v)
                    {
                        booleans.insert(origin_id, false);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlotFlagWarning {
    pub section: String,
    pub message: String,
}

pub fn validate_plot_flags(
    booleans: &BTreeMap<u16, bool>,
    integers: &BTreeMap<u16, i32>,
) -> Vec<PlotFlagWarning> {
    let mut warnings = Vec::new();
    let b = |id: u16| booleans.get(&id).copied().unwrap_or(false);
    let iv = |id: u16| integers.get(&id).copied().unwrap_or(0);

    // Origin uniqueness
    let origin_count = [2000u16, 2001, 2002, 2003, 2004, 2005]
        .iter()
        .filter(|&&id| b(id))
        .count();
    if origin_count > 1 {
        warnings.push(PlotFlagWarning {
            section: "Warden".into(),
            message: "Multiple origins are active — only one should be set.".into(),
        });
    }
    if b(2000) && ![2, 3].contains(&iv(1001)) {
        warnings.push(PlotFlagWarning {
            section: "Warden".into(),
            message: "Circle Mage origin requires an elf or human Warden race.".into(),
        });
    }
    if (b(2001) || b(2002)) && iv(1001) != 1 {
        warnings.push(PlotFlagWarning {
            section: "Warden".into(),
            message: "Dwarf origins require Warden race to be Dwarf.".into(),
        });
    }
    if (b(2003) || b(2004)) && iv(1001) != 2 {
        warnings.push(PlotFlagWarning {
            section: "Warden".into(),
            message: "Elf origins require Warden race to be Elf.".into(),
        });
    }
    if b(2005) && iv(1001) != 3 {
        warnings.push(PlotFlagWarning {
            section: "Warden".into(),
            message: "Human Noble origin requires Warden race to be Human.".into(),
        });
    }

    // Political marriage identity
    if b(2026) && !(iv(1000) == 2 && iv(1001) == 3 && b(2005)) {
        warnings.push(PlotFlagWarning {
            section: "Landsmeet".into(),
            message: "Alistair + Warden marriage will force a female human noble Warden.".into(),
        });
    }
    if b(2024) && !(iv(1000) == 1 && iv(1001) == 3 && b(2005)) {
        warnings.push(PlotFlagWarning {
            section: "Landsmeet".into(),
            message: "Anora + Warden marriage will force a male human noble Warden.".into(),
        });
    }

    // Alistair/Loghain contradictions
    let alistair_king = b(2020) || b(2021) || b(2026);
    if alistair_king && b(2022) {
        warnings.push(PlotFlagWarning {
            section: "Landsmeet".into(),
            message: "Alistair cannot be both king and exiled.".into(),
        });
    }
    if alistair_king && b(2023) {
        warnings.push(PlotFlagWarning {
            section: "Landsmeet".into(),
            message: "Alistair cannot be both king and executed.".into(),
        });
    }
    if b(2022) && b(2023) {
        warnings.push(PlotFlagWarning {
            section: "Landsmeet".into(),
            message: "Alistair cannot be both exiled and executed.".into(),
        });
    }
    if b(2025) && b(2097) {
        warnings.push(PlotFlagWarning {
            section: "Landsmeet".into(),
            message: "Loghain cannot be both killed and alive.".into(),
        });
    }

    // Archdemon killer
    let archdemon_count = [2028u16, 2029, 2030].iter().filter(|&&id| b(id)).count();
    if archdemon_count > 1 {
        warnings.push(PlotFlagWarning {
            section: "Finale".into(),
            message: "More than one Archdemon killer is set.".into(),
        });
    }
    if archdemon_count == 0 {
        warnings.push(PlotFlagWarning {
            section: "Finale".into(),
            message: "No Archdemon killer is set.".into(),
        });
    }

    // Ritual/Loghain consistency
    if !b(2104) && b(2029) && b(2097) {
        warnings.push(PlotFlagWarning {
            section: "Landsmeet".into(),
            message: "No ritual — Loghain killed the Archdemon and should be dead, but is also marked alive.".into(),
        });
    }
    if b(2104) && b(2029) && !b(2097) {
        warnings.push(PlotFlagWarning {
            section: "Landsmeet".into(),
            message: "Loghain killed the Archdemon (with ritual) — he should be marked as living."
                .into(),
        });
    }

    // Leliana
    if b(2038) && b(2045) {
        warnings.push(PlotFlagWarning {
            section: "Leliana".into(),
            message: "Leliana is marked as both recruited/stayed and attacked the Warden.".into(),
        });
    }
    if b(2038) && b(2044) {
        warnings.push(PlotFlagWarning {
            section: "Leliana".into(),
            message: "Leliana cannot be both recruited/stayed and not recruited.".into(),
        });
    }
    if b(2042) && !b(2038) {
        warnings.push(PlotFlagWarning {
            section: "Leliana".into(),
            message: "Leliana romance is active but she is not marked as recruited/stayed.".into(),
        });
    }

    // Zevran
    let zevran_gone = [2050u16, 2051, 2105, 2106, 2107, 2108]
        .iter()
        .any(|&id| b(id));
    if b(2039) && zevran_gone {
        warnings.push(PlotFlagWarning {
            section: "Zevran".into(),
            message: "Zevran is marked as recruited/stayed but also has a dead, left, or hostile flag set.".into(),
        });
    }
    if b(2048) && !b(2039) {
        warnings.push(PlotFlagWarning {
            section: "Zevran".into(),
            message: "Zevran romance is active but he is not marked as recruited/stayed.".into(),
        });
    }

    // Isabela
    if (b(2053) || b(2055)) && !b(2038) {
        warnings.push(PlotFlagWarning {
            section: "Isabela".into(),
            message:
                "Isabela + Leliana encounter requires Leliana to have been recruited and stayed."
                    .into(),
        });
    }
    if (b(2054) || b(2055)) && !b(2039) {
        warnings.push(PlotFlagWarning {
            section: "Isabela".into(),
            message:
                "Isabela + Zevran encounter requires Zevran to have been recruited and stayed."
                    .into(),
        });
    }

    // Warden's Keep
    if b(2070) && b(2071) {
        warnings.push(PlotFlagWarning {
            section: "Warden's Keep (DLC)".into(),
            message: "Avernus cannot have both ethical and evil research active simultaneously."
                .into(),
        });
    }
    if b(2068) && (b(2070) || b(2071)) {
        warnings.push(PlotFlagWarning {
            section: "Warden's Keep (DLC)".into(),
            message: "Avernus research flags are set but Avernus was killed.".into(),
        });
    }
    if !b(2094) && (b(2067) || b(2068) || b(2070) || b(2071)) {
        warnings.push(PlotFlagWarning {
            section: "Warden's Keep (DLC)".into(),
            message: "Warden's Keep was not started but Sophia/Avernus outcome flags are set."
                .into(),
        });
    }

    // Awakening
    if b(2063) && b(2096) {
        warnings.push(PlotFlagWarning {
            section: "Vigil's Keep".into(),
            message: "The Architect cannot be both killed and spared.".into(),
        });
    }
    if b(2065) && b(2084) {
        warnings.push(PlotFlagWarning {
            section: "Nathaniel".into(),
            message:
                "Nathaniel is marked as both recruited/stayed and died at the Vigil's Keep siege."
                    .into(),
        });
    }
    if b(2064) && b(2066) {
        warnings.push(PlotFlagWarning {
            section: "Anders".into(),
            message:
                "Anders is marked as both recruited/stayed and died at the Vigil's Keep siege."
                    .into(),
        });
    }

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bools(pairs: &[(u16, bool)]) -> BTreeMap<u16, bool> {
        pairs.iter().copied().collect()
    }

    fn ints(pairs: &[(u16, i32)]) -> BTreeMap<u16, i32> {
        pairs.iter().copied().collect()
    }

    fn warns_in(warnings: &[PlotFlagWarning], section: &str, fragment: &str) -> bool {
        warnings
            .iter()
            .any(|w| w.section == section && w.message.contains(fragment))
    }

    #[test]
    fn tc01_multiple_origins() {
        let w = validate_plot_flags(&bools(&[(2000, true), (2001, true)]), &ints(&[]));
        assert!(warns_in(&w, "Warden", "Multiple origins"));
    }

    #[test]
    fn tc02_single_origin_clean() {
        let w = validate_plot_flags(&bools(&[(2000, true)]), &ints(&[]));
        assert!(!warns_in(&w, "Warden", "Multiple origins"));
    }

    #[test]
    fn tc03_alistair_warden_marriage_wrong_identity() {
        let w = validate_plot_flags(&bools(&[(2026, true)]), &ints(&[(1000, 1), (1001, 3)]));
        assert!(warns_in(&w, "Landsmeet", "female human noble"));
    }

    #[test]
    fn tc04_alistair_warden_marriage_correct_identity() {
        let w = validate_plot_flags(
            &bools(&[(2026, true), (2005, true)]),
            &ints(&[(1000, 2), (1001, 3)]),
        );
        assert!(!warns_in(&w, "Landsmeet", "female human noble"));
    }

    #[test]
    fn tc04a_origin_must_match_race() {
        let w = validate_plot_flags(&bools(&[(2005, true)]), &ints(&[(1001, 2)]));
        assert!(warns_in(&w, "Warden", "Human Noble origin requires"));
    }

    #[test]
    fn tc04b_political_marriage_explains_backend_forced_identity() {
        let w = validate_plot_flags(
            &bools(&[(2024, true), (2004, true)]),
            &ints(&[(1000, 1), (1001, 2)]),
        );
        assert!(warns_in(&w, "Landsmeet", "will force a male human noble"));
    }

    #[test]
    fn tc05_alistair_king_and_exiled() {
        let w = validate_plot_flags(&bools(&[(2021, true), (2022, true)]), &ints(&[]));
        assert!(warns_in(&w, "Landsmeet", "exiled"));
    }

    #[test]
    fn tc06_loghain_killed_and_alive() {
        let w = validate_plot_flags(&bools(&[(2025, true), (2097, true)]), &ints(&[]));
        assert!(warns_in(&w, "Landsmeet", "killed and alive"));
    }

    #[test]
    fn tc07_no_archdemon_killer() {
        let w = validate_plot_flags(&bools(&[]), &ints(&[]));
        assert!(warns_in(&w, "Finale", "No Archdemon killer"));
    }

    #[test]
    fn tc08_multiple_archdemon_killers() {
        let w = validate_plot_flags(&bools(&[(2028, true), (2030, true)]), &ints(&[]));
        assert!(warns_in(&w, "Finale", "More than one"));
    }

    #[test]
    fn tc09_loghain_kills_no_ritual_marked_alive() {
        let w = validate_plot_flags(&bools(&[(2029, true), (2097, true)]), &ints(&[]));
        assert!(warns_in(&w, "Landsmeet", "should be dead"));
    }

    #[test]
    fn tc10_loghain_kills_ritual_not_marked_alive() {
        let w = validate_plot_flags(&bools(&[(2029, true), (2104, true)]), &ints(&[]));
        assert!(warns_in(&w, "Landsmeet", "should be marked as living"));
    }

    #[test]
    fn tc11_leliana_romance_not_recruited() {
        let w = validate_plot_flags(&bools(&[(2042, true)]), &ints(&[]));
        assert!(warns_in(&w, "Leliana", "not marked as recruited"));
    }

    #[test]
    fn tc12_zevran_romance_not_recruited() {
        let w = validate_plot_flags(&bools(&[(2048, true)]), &ints(&[]));
        assert!(warns_in(&w, "Zevran", "not marked as recruited"));
    }

    #[test]
    fn tc13_zevran_recruited_and_hostile() {
        let w = validate_plot_flags(&bools(&[(2039, true), (2050, true)]), &ints(&[]));
        assert!(warns_in(&w, "Zevran", "hostile"));
    }

    #[test]
    fn tc14_isabela_leliana_threesome_not_recruited() {
        let w = validate_plot_flags(&bools(&[(2053, true)]), &ints(&[]));
        assert!(warns_in(&w, "Isabela", "Leliana"));
    }

    #[test]
    fn tc15_isabela_foursome_zevran_not_recruited() {
        let w = validate_plot_flags(&bools(&[(2055, true), (2038, true)]), &ints(&[]));
        assert!(warns_in(&w, "Isabela", "Zevran"));
    }

    #[test]
    fn tc16_avernus_both_research() {
        let w = validate_plot_flags(
            &bools(&[(2094, true), (2070, true), (2071, true)]),
            &ints(&[]),
        );
        assert!(warns_in(&w, "Warden's Keep (DLC)", "both ethical and evil"));
    }

    #[test]
    fn tc17_avernus_research_while_killed() {
        let w = validate_plot_flags(
            &bools(&[(2094, true), (2068, true), (2071, true)]),
            &ints(&[]),
        );
        assert!(warns_in(&w, "Warden's Keep (DLC)", "Avernus was killed"));
    }

    #[test]
    fn tc18_wardens_keep_outcomes_without_started() {
        let w = validate_plot_flags(&bools(&[(2067, true)]), &ints(&[]));
        assert!(warns_in(&w, "Warden's Keep (DLC)", "not started"));
    }

    #[test]
    fn tc19_architect_killed_and_spared() {
        let w = validate_plot_flags(&bools(&[(2063, true), (2096, true)]), &ints(&[]));
        assert!(warns_in(&w, "Vigil's Keep", "killed and spared"));
    }

    #[test]
    fn tc20_anders_recruited_and_died() {
        let w = validate_plot_flags(&bools(&[(2064, true), (2066, true)]), &ints(&[]));
        assert!(warns_in(&w, "Anders", "died"));
    }

    #[test]
    fn tc21_canonical_valid_state_no_warnings() {
        let w = validate_plot_flags(
            &bools(&[
                (2005, true),
                (2020, true),
                (2030, true),
                (2104, true),
                (2097, true),
                (2038, true),
                (2039, true),
                (2094, true),
                (2067, true),
                (2071, true),
            ]),
            &ints(&[(1000, 2), (1001, 3)]),
        );
        // Should have no warnings except possibly the "No Archdemon killer" one
        // (2030 is set so that warning should be absent)
        assert!(!warns_in(&w, "Finale", "No Archdemon killer"));
        assert!(!warns_in(&w, "Warden", "Multiple origins"));
        assert!(!warns_in(&w, "Warden's Keep (DLC)", ""));
    }
}
