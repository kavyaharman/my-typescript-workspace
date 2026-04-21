import Aura from "@primeuix/themes/Aura";

export default defineNuxtConfig({
  modules: [
    '@primevue/nuxt-module'
  ],
  vite: {
    optimizeDeps: {
      include: [
        '@vue/devtools-core',
        '@vue/devtools-kit',
      ]
    }
  },
  primevue: {
        options: {
            ripple: true,
            inputVariant: 'filled',
            theme: {
                preset: Aura,
                options: {
                    prefix: 'p',
                    darkModeSelector: 'system',
                    cssLayer: false
                }
            }
        }
    },
  css: [
    'primeicons/primeicons.css'
  ]
})