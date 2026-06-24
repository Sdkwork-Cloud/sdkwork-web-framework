export type DevAuthClaims = {
  tenant_id?: string;
  permission_scope?: string;
};

/** Decode unsigned JWT payload for local dev console permission gating only. */
export function readDevAuthClaims(authToken: string | null | undefined): DevAuthClaims | null {
  if (!authToken?.trim()) {
    return null;
  }
  const parts = authToken.trim().split(".");
  if (parts.length < 2) {
    return null;
  }
  try {
    const normalized = parts[1].replace(/-/g, "+").replace(/_/g, "/");
    const padded = normalized.padEnd(normalized.length + ((4 - (normalized.length % 4)) % 4), "=");
    const json = atob(padded);
    return JSON.parse(json) as DevAuthClaims;
  } catch {
    return null;
  }
}

export function hasPermission(
  claims: DevAuthClaims | null,
  permission: string,
): boolean {
  if (!claims?.permission_scope) {
    return false;
  }
  return claims.permission_scope
    .split(",")
    .map((value) => value.trim())
    .filter(Boolean)
    .includes(permission);
}

export const PERM_CONTROL_PLANE = "web-framework.control-plane";
export const PERM_TENANT_ADMIN = "web-framework.tenant.admin";
export const PERM_PLATFORM_READ = "web-framework.platform.read";
