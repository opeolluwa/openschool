/**
 * Title-cases a person's name for display. Every word boundary (string start,
 * space, hyphen, apostrophe) gets its first letter uppercased; the rest of each
 * word is left exactly as entered, so "mary-jane o'neil" reads "Mary-Jane
 * O'Neil" and "McDonald" is untouched. Nullish/empty input yields "".
 */
export function capitalizeName(value?: string | null): string {
  if (!value) return "";

  return value.replace(
    /(^|[\s'-])(\p{Ll})/gu,
    (_, sep: string, ch: string) => sep + ch.toUpperCase(),
  );
}

/**
 * Composes a display name from the two optional parts the api stores,
 * capitalizing each. Returns `fallback` when neither part has content.
 */
export function formatPersonName(
  firstName?: string | null,
  lastName?: string | null,
  fallback = "",
): string {
  const name = [capitalizeName(firstName), capitalizeName(lastName)]
    .filter(Boolean)
    .join(" ")
    .trim();

  return name || fallback;
}
