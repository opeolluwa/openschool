import type { EmailListParams, NotificationListParams } from "~/types/inbox";
import type { OrderStatus } from "~/typegen/OrderStatus";

/**
 * Every cache key in the console, in one factory.
 *
 * Keys are the API for invalidation: a mutation says "everything about orders is
 * suspect" and the key hierarchy decides how much that costs. Each entity
 * therefore nests under its own name with explicit `list`/`detail` segments, so
 * `invalidateQueries({ queryKey: ordersKeys.all })` can refresh every order read
 * while `ordersKeys.list({...})` targets one exact page.
 *
 * Building keys here rather than inline means a typo becomes a type error instead
 * of a silent cache miss — an unrecognised key resolves to nothing, so a
 * misspelt `invalidateQueries` fails by quietly doing nothing at all.
 */
export const configKey = ["config"] as const;

/**
 * Countries are reference data: small, read on most screens, and changed by
 * nobody mid-session.
 */
export const countriesKey = ["countries"] as const;

/**
 * Roles are read on most screens, but only the list is reference data — the
 * per-role reads carry permissions, and an admin can add or remove one of
 * those mid-session. The list therefore keeps the reference window while the
 * per-role entries are keyed by role and cached like any other entity read.
 */
export const rolesKeys = {
  all: ["roles"] as const,
  list: ["roles", "list"] as const,
  /**
   * The signed-in admin's own role.
   *
   * Nested under `roles` so a role change invalidates it with everything else,
   * but kept out of `list`: it is read once per session by the shell, and it is
   * the one role read that cannot be fetched by identifier.
   */
  mine: ["roles", "me"] as const,
  /**
   * A single role on its own.
   *
   * Kept apart from `detail` because the two reads answer with different
   * shapes — the role alone, and the role with its permissions — and a shared
   * cache key would hand one shape to whoever asked for the other.
   */
  role: (identifier: string) => ["roles", "role", identifier] as const,
  detail: (identifier: string) => ["roles", "detail", identifier] as const,
  permissions: (identifier: string) =>
    ["roles", "detail", identifier, "permissions"] as const,
} as const;
export const productsKey = ["products"] as const;

export interface ProductListParams {
  page: number;
  pageSize: number;
}

export const productsKeys = {
  all: ["products"] as const,
  lists: ["products", "list"] as const,
  list: (params: ProductListParams) => ["products", "list", params] as const,
} as const;

/** The filters that fully determine a page of orders. */
export interface OrderListParams {
  page: number;
  pageSize: number;
  /** `null` means "no status filter", and is distinct from `undefined`. */
  status: OrderStatus | null;
}

export const ordersKeys = {
  all: ["orders"] as const,
  list: (params: OrderListParams) => ["orders", "list", params] as const,
  detail: (identifier: string) => ["orders", "detail", identifier] as const,
} as const;

export const complaintsKeys = {
  all: ["complaints"] as const,
  list: ["complaints", "list"] as const,
  detail: (identifier: string) => ["complaints", "detail", identifier] as const,
  /**
   * Nested under the complaint rather than sitting beside it, so deleting a
   * complaint cannot leave its reply thread behind as a live cache entry.
   */
  replies: (identifier: string) =>
    ["complaints", "detail", identifier, "replies"] as const,
} as const;

export interface CustomerListParams {
  page: number;
  pageSize: number;
}

export const customersKeys = {
  all: ["customers"] as const,
  reference: ["customers", "reference"] as const,
  lists: ["customers", "list"] as const,
  list: (params: CustomerListParams) => ["customers", "list", params] as const,
  detail: (identifier: string) => ["customers", "detail", identifier] as const,
} as const;

export const inventoryKeys = {
  all: ["inventory"] as const,
  /** Prefix matching every cached listing, for post-write invalidation. */
  lists: ["inventory", "list"] as const,
  list: (params: { page: number; pageSize: number }) =>
    ["inventory", "list", params] as const,
  detail: (identifier: string) => ["inventory", "detail", identifier] as const,
  count: ["inventory", "count"] as const,
} as const;

export const emailsKeys = {
  all: ["emails"] as const,
  /**
   * Prefix matching every cached listing, for `invalidateQueries` after a write.
   *
   * `emailsKeys.all` would work too but would drag the detail entry and the
   * polled badge along with it — a page change or a star toggle cannot affect
   * either, and refetching them would be a request with no possible effect.
   */
  lists: ["emails", "list"] as const,
  list: (params: EmailListParams) => ["emails", "list", params] as const,
  detail: (identifier: string) => ["emails", "detail", identifier] as const,
  /**
   * Deliberately outside `emailsKeys.all`: the unread badge is polled on a timer
   * and invalidating it alongside the listing would turn every page change into a
   * second request for a number that a page change cannot affect.
   */
  unreadCount: ["emails", "unread-count"] as const,
  /**
   * The whole-inbox total, which decides whether the inbox has ever had mail at
   * all. It is not the unread count: an inbox whose mail has all been read still
   * has mail, and reading emptiness off the unread figure empties the inbox in
   * the UI while the listing returns rows beside it.
   *
   * Kept beside the badge rather than under `lists` for the same reason, with one
   * difference: nothing in the console creates or deletes an email, so no write
   * can invalidate it and it never needs refetching on settle.
   */
  totalCount: ["emails", "total-count"] as const,
} as const;

export const notificationsKeys = {
  all: ["notifications"] as const,
  /** Prefix matching every cached listing; see `emailsKeys.lists`. */
  lists: ["notifications", "list"] as const,
  list: (params: NotificationListParams) =>
    ["notifications", "list", params] as const,
  /** Header panel preview; a different page size, so its own cache entry. */
  panel: ["notifications", "panel"] as const,
  unreadCount: ["notifications", "unread-count"] as const,
} as const;

export const teamsKeys = {
  all: ["teams"] as const,
  list: ["teams", "list"] as const,
} as const;

export const invitationsKeys = {
  all: ["invitations"] as const,
  list: ["invitations", "list"] as const,
  detail: (identifier: string) =>
    ["invitations", "detail", identifier] as const,
} as const;

export const uploadsKeys = {
  all: ["uploads"] as const,
  list: ["uploads", "list"] as const,
} as const;

/**
 * Keyed by window rather than made a single entry: the dashboard lets an admin
 * pick the trend range, and caching 24 months against a 12-month request would
 * show one window's figures under the other's label.
 */
export const metricsKey = (months: number) => ["metrics", months] as const;

export const invoicesKeys = {
  all: ["invoices"] as const,
  list: ["invoices", "list"] as const,
  detail: (identifier: string) => ["invoices", "detail", identifier] as const,
} as const;

export interface TransactionListParams {
  page: number;
  pageSize: number;
}

export const transactionsKeys = {
  all: ["transactions"] as const,
  lists: ["transactions", "list"] as const,
  list: (params: TransactionListParams) =>
    ["transactions", "list", params] as const,
  detail: (identifier: string) =>
    ["transactions", "detail", identifier] as const,
} as const;

export const notificationPreferencesKey = ["notification-preferences"] as const;
