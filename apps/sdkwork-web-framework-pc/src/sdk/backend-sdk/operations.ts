/** AUTO-GENERATED from apis/backend-api/web-framework/routes.manifest.json — do not edit manually. */
/** Regenerate: node scripts/generate-pc-admin-operations.mjs */
export const WEB_FRAMEWORK_ADMIN_API_PREFIX = "/backend/v3/api/web-framework";

export const webFrameworkAdminOperations = {
  corsPolicies: {
    list: `${WEB_FRAMEWORK_ADMIN_API_PREFIX}/cors-policies`,
    upsert: `${WEB_FRAMEWORK_ADMIN_API_PREFIX}/cors-policies`,
  },
  rateLimitPolicies: {
    list: `${WEB_FRAMEWORK_ADMIN_API_PREFIX}/rate-limit-policies`,
    upsert: `${WEB_FRAMEWORK_ADMIN_API_PREFIX}/rate-limit-policies`,
  },
  tenantRuntimeProfiles: {
    list: `${WEB_FRAMEWORK_ADMIN_API_PREFIX}/tenant-runtime-profiles`,
    upsert: `${WEB_FRAMEWORK_ADMIN_API_PREFIX}/tenant-runtime-profiles`,
  },
  securityEvents: {
    list: `${WEB_FRAMEWORK_ADMIN_API_PREFIX}/security-events`,
  },
  auditEvents: {
    list: `${WEB_FRAMEWORK_ADMIN_API_PREFIX}/audit-events`,
  },
  controlNodes: {
    list: `${WEB_FRAMEWORK_ADMIN_API_PREFIX}/control-nodes`,
    register: `${WEB_FRAMEWORK_ADMIN_API_PREFIX}/control-nodes`,
    heartbeat: (nodeId: string) =>
      `${WEB_FRAMEWORK_ADMIN_API_PREFIX}/control-nodes/${encodeURIComponent(nodeId)}/heartbeat`,
    delete: (nodeId: string) =>
      `${WEB_FRAMEWORK_ADMIN_API_PREFIX}/control-nodes/${encodeURIComponent(nodeId)}`,
  },
  runtimeDefaults: {
    snapshot: `${WEB_FRAMEWORK_ADMIN_API_PREFIX}/runtime-defaults`,
  },
  optionalFeatures: {
    snapshot: `${WEB_FRAMEWORK_ADMIN_API_PREFIX}/optional-features`,
  },
} as const;

export const webFrameworkAdminOperationIds = {
  corsPoliciesList: "webFramework.corsPolicies.list",
  corsPoliciesUpsert: "webFramework.corsPolicies.upsert",
  rateLimitPoliciesList: "webFramework.rateLimitPolicies.list",
  rateLimitPoliciesUpsert: "webFramework.rateLimitPolicies.upsert",
  tenantRuntimeProfilesList: "webFramework.tenantRuntimeProfiles.list",
  tenantRuntimeProfilesUpsert: "webFramework.tenantRuntimeProfiles.upsert",
  securityEventsList: "webFramework.securityEvents.list",
  auditEventsList: "webFramework.auditEvents.list",
  controlNodesList: "webFramework.controlNodes.list",
  controlNodesRegister: "webFramework.controlNodes.register",
  controlNodesHeartbeat: "webFramework.controlNodes.heartbeat",
  controlNodesDelete: "webFramework.controlNodes.delete",
  runtimeDefaultsSnapshot: "webFramework.runtimeDefaults.snapshot",
  optionalFeaturesSnapshot: "webFramework.optionalFeatures.snapshot",
} as const;
