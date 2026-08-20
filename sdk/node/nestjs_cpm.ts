/**
 * CPM NestJS Decorator & Provider Helper
 *
 * Usage:
 *   import { CpmBridgeProvider } from './sdk/node/nestjs_cpm';
 *
 *   @Module({
 *     providers: [CpmBridgeProvider],
 *   })
 *   export class AppModule {}
 */

import { CpmBridge } from './cpm_sdk';

export const CPM_BRIDGE_TOKEN = 'CPM_BRIDGE_TOKEN';

export const CpmBridgeProvider = {
  provide: CPM_BRIDGE_TOKEN,
  useFactory: () => new CpmBridge(),
};
