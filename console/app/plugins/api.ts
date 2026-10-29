import axios from "axios";
import { useBackendStore } from "~/stores/backend";
import { useTokenStore } from "~/stores/token";

const NETWORK_REQUEST_TIMEOUT = 27500;
const UPLOADS_TIMEOUT = 300000;
const UPLOAD_LIMIT_SIZE = 1024 * 1024 * 25;

/**
 * Endpoints whose own failures must not be read as "session expired".
 *
 * These legitimately answer 401 while the caller is not signed in — wrong
 * credentials, a dead refresh token, or an expired reset link — and treating
 * that as a dead session would wipe state and bounce the user away from the
 * very screen where they are trying to authenticate.
 */
const AUTH_ENTRY_ENDPOINTS = new Set([
  "/login",
  "/refresh-token",
  "/logout",
  "/reset-password",
  "/verify-otp",
  "/request-otp",
  "/users/two-factor/otp",
  "/users/two-factor",
  "/users/password/change",
  "/users/password",
]);

export default defineNuxtPlugin(() => {
  const config = useRuntimeConfig();

  const api = axios.create({
    baseURL: String(config.public.apiBaseUrl),
    headers: {
      Accept: "application/json",
    },
    timeout: NETWORK_REQUEST_TIMEOUT,
  });

  api.interceptors.request.use(
    async (request) => {
      const tokenStore = useTokenStore();

      // Never attempt a refresh while calling the refresh endpoint.
      if (request.url !== "/refresh-token") {
        if (!tokenStore.isAccessTokenValid()) {
          await tokenStore.getRefreshToken(api);
        }
      }

      const token = tokenStore.accessToken;

      if (token && !request.headers.Authorization) {
        request.headers.Authorization = `Bearer ${token}`;
      }

      return request;
    },
    (error) => Promise.reject(error),
  );

  /**
   * Ends the session when the API rejects the token.
   *
   * Without this the request fails but the console keeps going, and the query
   * cache is left holding the previous admin's orders, customers and unread
   * counts — visible to whoever opens the console next on a shared machine. An
   * expired session has to take the cached data down with it.
   *
   * Guarded so only the first rejection acts: a dashboard's worth of queries all
   * fail together, and each one independently clearing the cache and navigating
   * would thrash the router on the way out.
   */
  let endingSession = false;

  function endSession() {
    if (endingSession) return;

    endingSession = true;

    const nuxtApp = useNuxtApp();

    useTokenStore().$reset();

    void nuxtApp.runWithContext(async () => {
      try {
        await navigateTo("/auth/login", { replace: true });
      } finally {
        endingSession = false;
      }
    });
  }

  api.interceptors.response.use(
    (response) => {
      useBackendStore().markReachable();

      return response;
    },
    (error) => {
      // A transport failure means the API is not answering, so surface it
      // globally instead of leaving each caller to decode "Network Error".
      if (isBackendUnreachable(error)) {
        useBackendStore().markUnreachable();
      }

      const status = getApiErrorStatus(error);
      const message = getApiErrorMessage(error, "Unknown error");

      if (
        status === 401 &&
        !AUTH_ENTRY_ENDPOINTS.has(String(error?.config?.url))
      ) {
        endSession();
      }

      // The status is carried on the rejection rather than dropped, so a caller
      // can tell an expired session from a server fault. See `ApiError`.
      return Promise.reject(new ApiError(message, status));
    },
  );

  return {
    provide: {
      api,
    },
  };
});

export { UPLOADS_TIMEOUT, UPLOAD_LIMIT_SIZE };
