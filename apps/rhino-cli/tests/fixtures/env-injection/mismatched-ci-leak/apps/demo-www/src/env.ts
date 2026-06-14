export const env = createEnv({
  server: {
    DEMO_WWW_API_URL: z.string(),
    WEB_BASE_URL: z.string(),
  },
  experimental__runtimeEnv: {},
});
