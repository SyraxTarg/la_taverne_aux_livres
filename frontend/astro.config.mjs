// @ts-check
import { defineConfig, envField } from 'astro/config';

export default defineConfig({
  env: {
    schema: {
      API_URL: envField.string({
        context: 'server',
        access: 'secret',
        default: 'http://127.0.0.1',
      }),
    },
  },
});