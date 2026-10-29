/**
 * Timestamp parsing for the API's date fields.
 *
 * The API returns RFC 3339 instants (e.g. `"2026-01-02T03:04:05+00:00"`), which
 * `new Date(value)` parses directly; the offset pins the instant and the
 * browser renders it in the viewer's local zone.
 *
 * Legacy fields may still arrive as a naive SeaORM `timestamp` string
 * (`"2026-01-02 03:04:05"`) — a space separator and no offset. That is not
 * RFC 3339, so `new Date(value)` would be relying on an implementation detail:
 * V8 accepts it, but nothing guarantees it, and Safari is stricter about
 * non-ISO input than V8. Rewriting the separator to `T` keeps the value
 * recognisably a datetime; having no offset, it is read as local time.
 */
const SPACE_SEPARATED_DATETIME = /^(\d{4}-\d{2}-\d{2}) (\d{2}:\d{2}:\d{2})$/;

export function parseApiDate(value: string | null | undefined): Date | null {
  if (!value) return null;

  const normalised = value.replace(SPACE_SEPARATED_DATETIME, "$1T$2");

  const date = new Date(normalised);

  return Number.isNaN(date.getTime()) ? null : date;
}

/**
 * Compact form for list rows: the time for something placed today, the day and
 * month for anything older. An unparseable value renders as an em dash rather
 * than as `Invalid Date`, so a malformed timestamp cannot masquerade as a real
 * date in a column an admin is scanning.
 */
export function formatListDate(value: string | null | undefined): string {
  const date = parseApiDate(value);

  if (!date) return "—";

  const now = new Date();

  if (date.toDateString() === now.toDateString()) {
    return date.toLocaleTimeString("en-US", {
      hour: "2-digit",
      minute: "2-digit",
      hour12: false,
    });
  }

  return date.toLocaleDateString("en-US", { day: "numeric", month: "short" });
}

/**
 * Full form for a detail header: `Wed, Dec 23, 2025 12:30 AM`.
 *
 * The date and time are formatted as two separate `toLocale*` calls on purpose.
 * `toLocaleString` glues them together with the locale's own date-time
 * separator, which inserts a stray comma ("Dec 23, 2025, 00:30") — and it has no
 * option to include the weekday at all.
 */
export function formatDateTime(value: string | null | undefined): string {
  const date = parseApiDate(value);

  if (!date) return "—";

  const day = date.toLocaleDateString("en-US", {
    weekday: "short",
    day: "numeric",
    month: "short",
    year: "numeric",
  });

  // `numeric` rather than `2-digit` so the hour is not zero-padded: 12:30 AM
  // and 1:05 PM, not 01:05 PM.
  const time = date.toLocaleTimeString("en-US", {
    hour: "numeric",
    minute: "2-digit",
    hour12: true,
  });

  return `${day} ${time}`;
}
