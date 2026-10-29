import type { OrderStatus } from "~/typegen/OrderStatus";

/**
 * The status vocabulary is one set of lowercase tokens shared by the enum the
 * API emits, the `?status=` filter it accepts, and the `PUT` body it writes, so
 * a value can be fed from one call straight into the next.
 */
export const ORDER_STATUSES: OrderStatus[] = [
  "pending",
  "paid",
  "fulfilled",
  "cancelled",
  "conflicted",
];

export const ORDER_STATUS_LABELS: Record<OrderStatus, string> = {
  pending: "Pending",
  paid: "Paid",
  fulfilled: "Fulfilled",
  cancelled: "Cancelled",
  conflicted: "Conflicted",
};

/**
 * `status` is nullable in the model but NOT NULL in the database, where it
 * defaults to `pending`. The console reads a missing value as `pending` so a
 * row can never render as a status it is not in.
 */
export function resolveOrderStatus(status: OrderStatus | null): OrderStatus {
  return status ?? "pending";
}

/**
 * Which statuses an admin may move an order to, keyed by where it is now.
 *
 * The API validates nothing — `PUT /orders/{id}` writes whatever status it is
 * given — so this table is the only thing standing between the console and an
 * order marked `fulfilled` without ever having been paid. That is why `paid` is
 * absent from every row: the Paystack webhook sets it, and only after the
 * settled amount and currency have been re-verified against the stored lines.
 * Setting it by hand would fake a payment.
 *
 * `pending → cancelled` and `paid → cancelled` are the two real exits, and
 * `paid → fulfilled` is the one an admin reaches for to say "this has gone
 * out". `fulfilled`, `cancelled` and `conflicted` are terminal.
 */
export const ORDER_STATUS_TRANSITIONS: Record<
  OrderStatus,
  readonly OrderStatus[]
> = {
  pending: ["cancelled"],
  paid: ["fulfilled", "cancelled"],
  fulfilled: [],
  cancelled: [],
  conflicted: [],
};

export function canTransitionOrder(
  from: OrderStatus | null,
  to: OrderStatus,
): boolean {
  return ORDER_STATUS_TRANSITIONS[resolveOrderStatus(from)].includes(to);
}

/**
 * Badge colours are keyed off meaning rather than off the status name, so the
 * two statuses that mean "money has not arrived" cannot drift apart visually
 * and `conflicted` cannot be mistaken for ordinary business.
 */
const STATUS_BADGE_CLASSES: Record<OrderStatus, string> = {
  pending: "bg-gray-100 text-gray-600 dark:bg-white/5 dark:text-white/50",
  paid: "bg-primary-50/20 text-primary-700 dark:bg-primary-500/10 dark:text-primary-400",
  fulfilled:
    "bg-green-50 text-green-700 dark:bg-green-500/10 dark:text-green-400",
  cancelled: "bg-red-50 text-red-700 dark:bg-red-500/10 dark:text-red-400",
  conflicted:
    "bg-amber-50 text-amber-700 dark:bg-amber-500/10 dark:text-amber-400",
};

/**
 * `conflicted` means money arrived but disagreed with the order, so it is
 * flagged rather than listed quietly: it needs a human to reconcile or refund,
 * and burying it in a routine list is how that gets missed.
 */
export function orderStatusBadgeClass(status: OrderStatus | null): string {
  return STATUS_BADGE_CLASSES[resolveOrderStatus(status)];
}

export function orderStatusLabel(status: OrderStatus | null): string {
  return ORDER_STATUS_LABELS[resolveOrderStatus(status)];
}
