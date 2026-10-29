/**
 * Freshness policy for every query in the console, in one place.
 *
 * These exist as named constants because the numbers encode a judgement about
 * each resource — how expensive it is to read, and how wrong a stale answer
 * would look — and that judgement is invisible once it is inlined as `300_000`
 * at a call site. A query that needs a different value overrides it locally, so
 * the exception is visible in the place it happens.
 *
 * Nothing here polls. Only the aggregate counts in the nav do, and they opt into
 * it individually via `refetchInterval`.
 */
export const QUERY_STALE_TIMES = {
  /**
   * Reference data: countries, roles, product catalogue.
   *
   * These change on the order of once a quarter but are read on nearly every
   * screen, so caching them for half an hour removes a request from every form
   * an admin opens without ever risking a visibly wrong dropdown.
   */
  reference: 30 * 60 * 1000,

  /**
   * Paginated listings and single-entity reads.
   *
   * Long enough that clicking into a row and back does not re-read the list,
   * short enough that a price or status change made elsewhere shows up within a
   * minute of returning to the tab.
   */
  listing: 60 * 1000,

  /**
   * Aggregate counts behind the nav badges.
   *
   * Shorter than a listing because they exist purely to be glanceable — a
   * number that is up to 30s behind is not a discrepancy anyone can perceive,
   * but the whole point of them is being current at a glance.
   */
  count: 30 * 1000,

  /**
   * Thread-like append-only reads, e.g. complaint replies.
   *
   * A reply thread changes when someone replies, which happens inside the
   * session that is reading it, and is driven by invalidation on write. Reading
   * the list and opening a complaint are separate steps an admin takes minutes
   * apart, so this can be much longer than a listing.
   */
  replies: 5 * 60 * 1000,

  /**
   * The signed-in admin's own profile.
   *
   * Shown in the shell on every page, but changed only by the person looking at
   * it — and that change goes through a mutation that invalidates it.
   */
  profile: 5 * 60 * 1000,

  /**
   * Dashboard aggregates (revenue, sales by category).
   *
   * Slightly longer than a listing: the figures are drawn as trends over months,
   * so a minute of drift is invisible at that scale, while re-computing every
   * chart on each nav would be the most expensive read in the app.
   */
  metrics: 2 * 60 * 1000,

  /**
   * The admin's own delivery preferences.
   *
   * Read on the settings screen and written from it. Nothing else displays them,
   * so the only reason to re-read is a change made in another tab.
   */
  preferences: 5 * 60 * 1000,
} as const;

/**
 * Polling interval for the unread badge counts.
 *
 * Only the counts poll, never the lists behind them: a count is one aggregate
 * row, so a minute of polling costs two cheap requests, whereas polling the
 * order listing would pull every row on the page over and over for no benefit.
 */
export const BADGE_POLL_INTERVAL = 60 * 1000;
