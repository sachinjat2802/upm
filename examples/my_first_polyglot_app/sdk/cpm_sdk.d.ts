/**
 * CPM Node.js / TypeScript SDK Type Definitions
 */

export interface BridgeCallResult {
    [key: string]: any;
}

export class CpmBridge {
    /**
     * Initialize the CPM Cross-Language RPC Bridge.
     * @param cpmBin Optional path to cpm executable binary.
     */
    constructor(cpmBin?: string);

    /**
     * Call a foreign language method over CPM stdio RPC bridge.
     * @param target Method target in format 'language:method' (e.g. 'python:math.sqrt')
     * @param args Arguments array
     */
    call<T = any>(target: string, args?: any[]): Promise<T>;

    /**
     * Inspect registered methods on a foreign language host.
     * @param language Language identifier ('python', 'node', 'rust')
     */
    inspect(language: string): Promise<string>;
}
