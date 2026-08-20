/**
 * CPM Nuxt 3 Module Integration
 *
 * Usage in nuxt.config.ts:
 *   export default defineNuxtConfig({
 *     modules: ['./sdk/node/nuxt_cpm.ts']
 *   })
 */

import { CpmBridge } from './cpm_sdk';

export default function cpmNuxtModule() {
  console.log('[Nuxt 3 CPM Module] Loaded CPM Polyglot Bridge Plugin');
  return {
    provide: {
      cpmBridge: new CpmBridge()
    }
  };
}
