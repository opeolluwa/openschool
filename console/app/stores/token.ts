import { defineStore } from "pinia";
import type { AxiosInstance } from "axios";

/**
 * The refresh currently in flight, if any.
 *
 * Deliberately module-scoped rather than store state. It is a promise, which
 * cannot be serialised, and the store is persisted — as `isRefreshing` was,
 * before this replaced it. A boolean in persisted state meant a tab closed
 * mid-refresh rehydrated `isRefreshing: true` and silently refused to refresh
 * again for the rest of that install's life.
 *
 * It is also what makes concurrent requests safe. Every request awaits this
 * promise instead of racing to issue its own refresh: a dashboard fires half a
 * dozen queries at once when it mounts, and the old early-return-on-flag sent
 * all but the first of them off with the expired access token, so they came back
 * 401. One refresh, many waiters.
 */
let refreshInFlight: Promise<boolean> | null = null;

export const useTokenStore = defineStore("token_store", {
  state: () => ({
    accessToken: "",
    refreshToken: "",
    accessTokenExpiry: 0,
    refreshTokenExpiry: 0,
    transientToken: "",
  }),

  actions: {
    persistRefreshToken(refreshToken: string) {
      this.refreshToken = refreshToken;
    },

    getTransientToken() {
      return this.transientToken || null;
    },

    persistTransientToken(token: string) {
      this.transientToken = token;
    },

    clearTransientToken() {
      this.transientToken = "";
    },

    persistAccessToken(accessToken: string) {
      this.accessToken = accessToken;
    },

    clearTokens() {
      this.$reset();
    },

    setAccessTokenExpiry(expiry: number) {
      this.accessTokenExpiry = expiry;
    },

    setRefreshTokenExpiry(expiry: number) {
      this.refreshTokenExpiry = expiry;
    },

    extractAccessToken() {
      return this.accessToken;
    },

    /**
     * Refreshes the access token, coalescing concurrent callers.
     *
     * Returns whether a usable token is in place afterwards. Every caller waits
     * on the same in-flight attempt, so N simultaneous requests cost one refresh.
     */
    async getRefreshToken(client: AxiosInstance): Promise<boolean> {
      if (refreshInFlight) return refreshInFlight;

      if (!this.refreshToken) return false;

      refreshInFlight = (async () => {
        try {
          const res = await client.post("/refresh-token", null, {
            headers: { Authorization: `Bearer ${this.refreshToken}` },
          });

          const data = res.data;
          if (!data) return false;

          this.persistAccessToken(data.accessToken);
          this.setAccessTokenExpiry(data.accessTokenExpiry);
          this.persistRefreshToken(data.refreshToken);
          this.setRefreshTokenExpiry(data.refreshTokenExpiry);

          return true;
        } catch {
          this.clearTokens();
          return false;
        } finally {
          // Cleared before the promise settles, so the next request after a
          // failure starts a fresh attempt rather than reusing a rejected one.
          refreshInFlight = null;
        }
      })();

      return refreshInFlight;
    },

    isAccessTokenValid() {
      const now = Math.floor(Date.now() / 1000);
      return this.accessToken && this.accessTokenExpiry > now + 60;
    },

    isRefreshTokenValid() {
      const now = Math.floor(Date.now() / 1000);
      return this.refreshToken && this.refreshTokenExpiry > now + 60;
    },
  },

  getters: {},

  persist: true,
});
