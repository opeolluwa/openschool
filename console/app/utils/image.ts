/**
 * Largest image the API accepts. Mirrors `MAX_IMAGE_SIZE_BYTES` in
 * `backoffice-admin/src/utils/image.rs`; the server re-checks it, so this is
 * here to fail fast with a clear message rather than to be the only guard.
 */
export const MAX_IMAGE_SIZE_BYTES = 2 * 1024 * 1024;

export const IMAGE_SIZE_ERROR = "Image must be 2 MB or smaller.";

export function isImageTooLarge(file: File | null | undefined): boolean {
  return Boolean(file && file.size > MAX_IMAGE_SIZE_BYTES);
}
