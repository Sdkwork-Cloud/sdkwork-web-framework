import type { ApiEnvelope } from "../../api/types";

export type BackendAuthCredentials = {
  authToken: string;
  accessToken: string;
};

export type BackendSdkTransport = {
  get<T>(path: string): Promise<T>;
  put<T>(path: string, payload: unknown): Promise<T>;
  post<T>(path: string, payload?: unknown): Promise<T>;
  delete<T>(path: string): Promise<T>;
};

export class BackendSdkError extends Error {
  readonly status: number;
  readonly problemType?: string;
  readonly requestId?: string;
  readonly traceId?: string;

  constructor(
    message: string,
    status: number,
    problemType?: string,
    requestId?: string,
    traceId?: string,
  ) {
    super(message);
    this.name = "BackendSdkError";
    this.status = status;
    this.problemType = problemType;
    this.requestId = requestId;
    this.traceId = traceId;
  }
}

const DEV_AUTH_TOKEN_STORAGE_KEY = "sdkwork.authToken";

declare const process: {
  env: {
    SDKWORK_ACCESS_TOKEN?: string;
  };
};

function readDevAuthTokenFromSession(): string {
  if (typeof sessionStorage === "undefined") {
    return "";
  }
  return sessionStorage.getItem(DEV_AUTH_TOKEN_STORAGE_KEY)?.trim() ?? "";
}

/** Reads dual-token credentials for local dev: access from env, auth from session storage. */
export function readBackendAuthFromEnv(): BackendAuthCredentials | null {
  const accessToken = process.env.SDKWORK_ACCESS_TOKEN?.trim();
  const authToken = readDevAuthTokenFromSession();
  if (!authToken || !accessToken) {
    return null;
  }
  return { authToken, accessToken };
}

function dualTokenHeaders(credentials: BackendAuthCredentials): HeadersInit {
  return {
    Authorization: `Bearer ${credentials.authToken}`,
    "Access-Token": credentials.accessToken,
  };
}

/** Internal transport for backend SDK facades. UI code must consume SDK methods, not this layer. */
export function createBackendSdkTransport(
  baseUrl: string,
  credentials?: BackendAuthCredentials | null,
): BackendSdkTransport {
  const base = baseUrl.replace(/\/$/, "");
  const authHeaders = credentials ? dualTokenHeaders(credentials) : {};

  async function request<T>(path: string, init?: RequestInit): Promise<T> {
    const response = await fetch(`${base}${path}`, {
      headers: {
        "content-type": "application/json",
        ...authHeaders,
        ...(init?.headers ?? {}),
      },
      ...init,
    });
    const contentType = response.headers.get("content-type") ?? "";
    if (!response.ok) {
      if (contentType.includes("application/problem+json")) {
        const problem = (await response.json()) as {
          detail?: string;
          title?: string;
          type?: string;
          requestId?: string;
          traceId?: string;
        };
        throw new BackendSdkError(
          problem.detail ?? problem.title ?? `backend SDK request failed: ${response.status}`,
          response.status,
          problem.type,
          problem.requestId,
          problem.traceId,
        );
      }
      const fallback = (await response.json().catch(() => null)) as ApiEnvelope<T> | null;
      throw new BackendSdkError(
        fallback?.message ?? `backend SDK request failed: ${response.status}`,
        response.status,
      );
    }
    if (response.status === 204) {
      return undefined as T;
    }
    const body = (await response.json()) as ApiEnvelope<T>;
    if (!body.success) {
      throw new BackendSdkError(
        body.message ?? `backend SDK request failed: ${response.status}`,
        response.status,
      );
    }
    return body.data;
  }

  return {
    get: <T>(path: string) => request<T>(path),
    put: <T>(path: string, payload: unknown) =>
      request<T>(path, { method: "PUT", body: JSON.stringify(payload) }),
    post: <T>(path: string, payload?: unknown) =>
      request<T>(path, {
        method: "POST",
        body: payload === undefined ? undefined : JSON.stringify(payload),
      }),
    delete: <T>(path: string) => request<T>(path, { method: "DELETE" }),
  };
}

function query(params: Record<string, string | undefined>) {
  const search = new URLSearchParams();
  for (const [key, value] of Object.entries(params)) {
    if (value) {
      search.set(key, value);
    }
  }
  const rendered = search.toString();
  return rendered ? `?${rendered}` : "";
}

export { query };
