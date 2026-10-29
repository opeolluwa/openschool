const AXIOS_STATUS_MESSAGE = /^Request failed with status code \d+$/;
const HTML_BODY = /^\s*<(?:!doctype|html|head|body)/i;

function asRecord(value: unknown): Record<string, unknown> | undefined {
  return typeof value === "object" && value !== null && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : undefined;
}

function readMessage(payload: unknown): string | undefined {
  const record = asRecord(payload);

  if (record) {
    for (const key of ["message", "detail", "error"]) {
      const value = record[key];

      if (typeof value === "string" && value.trim()) {
        return value;
      }
    }

    for (const value of Object.values(record)) {
      if (Array.isArray(value) && value.length > 0) {
        const [first] = value;

        if (typeof first === "string" && first.trim()) {
          return first;
        }
      }
    }
  }

  if (
    typeof payload === "string" &&
    payload.trim() &&
    !HTML_BODY.test(payload)
  ) {
    return payload;
  }

  return undefined;
}

export const BACKEND_UNREACHABLE_MESSAGE =
  "Cannot reach the API. The backend may be restarting.";

/**
 * True when the request never produced an HTTP response.
 *
 * Axios only populates `error.response` once something came back, so its absence
 * means the connection failed, timed out, or — the case worth catching here —
 * the browser blocked the response because of CORS. A cross-origin failure is
 * reported by the browser as an opaque `Network Error` with no response and no
 * status, which is indistinguishable from a dead server. That ambiguity is why
 * a backend that was down for a rebuild looked like a CORS misconfiguration.
 */
export function isBackendUnreachable(error: unknown): boolean {
  const record = asRecord(error);

  if (!record) {
    return false;
  }

  if (asRecord(record.response)) {
    return false;
  }

  // A response-shaped object was rejected before reaching this check (for
  // example an interceptor that threw), so it is not a transport failure.
  if (record.status !== undefined || record.config !== undefined) {
    return false;
  }

  return typeof record.request === "object" && record.request !== null;
}

export function getApiErrorMessage(error: unknown, fallback: string): string {
  if (isBackendUnreachable(error)) {
    return BACKEND_UNREACHABLE_MESSAGE;
  }

  const record = asRecord(error);

  if (!record) {
    return fallback;
  }

  const response = asRecord(record.response);
  const fromResponse = readMessage(response?.data);
  if (fromResponse) {
    return fromResponse;
  }

  const fromData = readMessage(record.data);
  if (fromData) {
    return fromData;
  }

  if (
    typeof record.message === "string" &&
    record.message.trim() &&
    !AXIOS_STATUS_MESSAGE.test(record.message)
  ) {
    return record.message;
  }

  return fallback;
}

/**
 * The HTTP status an error carries, or `null` if the request never got one.
 *
 * `null` is the transport-failure case rather than a zero: the browser blocked
 * or dropped the response, so there is no status to report and the answer is
 * "the API is unreachable", not "the API said no".
 */
export function getApiErrorStatus(error: unknown): number | null {
  const status = asRecord(asRecord(error)?.response)?.status;

  return typeof status === "number" ? status : null;
}

/**
 * The HTTP status an error carries, whichever shape it arrives in.
 *
 * A rejected request reaches a caller as an `ApiError`, which carries the
 * status itself, while the raw axios error it was built from keeps it under
 * `response`. Deciding what to do about a failure should not depend on which
 * of the two happens to be in hand.
 */
export function getErrorStatus(error: unknown): number | null {
  const record = asRecord(error);

  if (record && typeof record.status === "number") {
    return record.status;
  }

  return getApiErrorStatus(error);
}

/**
 * A failed request, carrying the status the API answered with.
 *
 * The response interceptor used to reject with a bare `Error(message)`, which is
 * enough to show a toast and not enough to act on: once the status is gone there
 * is no way to tell "you are signed out, go to the login screen" from "the
 * server had a bad time, offer a retry". Status survives here so a query or a
 * mutation can branch on it.
 */
export class ApiError extends Error {
  readonly status: number | null;

  constructor(message: string, status: number | null) {
    super(message);

    this.name = "ApiError";
    this.status = status;
  }
}
