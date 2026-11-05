// https://nuxt.com/docs/api/configuration/nuxt-config
import { fileURLToPath } from 'node:url';
import tailwindcss from '@tailwindcss/vite';

export default defineNuxtConfig({
  modules: ['@nuxt/ui', '@pinia/nuxt'],
  devtools: { enabled: false },
  css: ['~/assets/styles/main.css'],
  components: {
    dirs: [{ path: '~/components', extensions: ['.vue'] }],
  },
  vite: {
    plugins: [tailwindcss()],
  },
  fonts: {
    providers: {
      google: false,
      googleicons: false,
    },
  },
  icon: {
    provider: 'none',
  },
  app: {
    head: {
      link: [{ rel: 'icon', type: 'image/svg+xml', href: '/icon/favicon.svg' }],
    },
  },
  alias: {
    '@domain': fileURLToPath(new URL('./domain', import.meta.url)),
    '@application': fileURLToPath(new URL('./application', import.meta.url)),
    '@interface-adapters': fileURLToPath(new URL('./interface-adapters', import.meta.url)),
    '@infrastructure': fileURLToPath(new URL('./infrastructure', import.meta.url)),
  },
  compatibilityDate: '2025-07-15',
});
