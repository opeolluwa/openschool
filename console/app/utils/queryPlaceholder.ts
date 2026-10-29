/**
 * `placeholderData` for a paginated screen: hold the previous page on screen
 * while the next one loads.
 *
 * TanStack removed the `keepPreviousData` sentinel in v5 in favour of a
 * `placeholderData` function, so this is the replacement. It is wrapped up and
 * named rather than inlined as `(previous) => previous` at each call site,
 * because the bare arrow does not say *why* the previous page is being kept, and
 * "why" is the part a future reader needs.
 *
 * Without it, changing page resets the data to `undefined`, the table empties,
 * and the screen reads as "no orders on this page" instead of "loading".
 */
export function keepPreviousPage<TPage>(
  previousData: TPage | undefined,
): TPage | undefined {
  return previousData;
}
