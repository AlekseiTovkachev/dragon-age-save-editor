import type { Character } from "../../types";

type CharacterHeaderProps = {
  character: Character;
  dirty: boolean;
};

function poolTotal(character: Character) {
  return Object.values(character.point_pools).reduce<number>((total, value) => total + (value ?? 0), 0);
}

export function CharacterHeader({ character, dirty }: CharacterHeaderProps) {
  const unspent = poolTotal(character);
  const detailLine = character.template_resref ? `Template ${character.template_resref}` : null;

  return (
    <header className="char-header">
      <div>
        <h2 className="char-name">{character.name}</h2>
        {detailLine ? <div className="char-class">{detailLine}</div> : null}
      </div>
      <div className="char-chips" aria-label="Character summary">
        <span className="chip gold">Level {character.level ?? "-"}</span>
        <span className="chip">{character.experience ?? "-"} XP</span>
        <span className={unspent > 0 ? "chip rune" : "chip"}>{unspent} unspent</span>
        {dirty ? <span className="chip dirty-chip">Modified</span> : null}
      </div>
    </header>
  );
}
