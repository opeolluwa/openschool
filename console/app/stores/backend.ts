import { defineStore } from "pinia";

/**
 * Tracks whether the API is currently answering.
 *
 * This exists because a backend that is down is otherwise almost invisible: the
 * browser reports it as an opaque CORS error, and persisted stores happily
 * rehydrate stale rows that render as if they were live.
 *
 * Intentionally not persisted. A persisted "unreachable" flag would survive a
 * reload and claim the API is down before a single request has been made.
 */
export const useBackendStore = defineStore("backend", {
  state: () => ({
    reachable: true,
  }),

  actions: {
    markReachable() {
      this.reachable = true;
    },

    markUnreachable() {
      this.reachable = false;
    },
  },
});
