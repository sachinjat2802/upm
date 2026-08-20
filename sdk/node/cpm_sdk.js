/**
 * CPM Node.js SDK — Native JavaScript/TypeScript Client for CPM / UPM Polyglot Bridge RPC
 *
 * Usage:
 *   const { CpmBridge } = require('./cpm_sdk');
 *   const bridge = new CpmBridge();
 *   const result = await bridge.call('python:math.sqrt', [144.0]);
 *   console.log(result);
 */

const { execFile } = require('child_process');
const path = require('path');
const fs = require('fs');

class CpmBridge {
    constructor(cpmBin) {
        this.cpmBin = cpmBin || this._findCpmBin();
    }

    _findCpmBin() {
        let curr = path.resolve(__dirname);
        for (let i = 0; i < 5; i++) {
            for (const rel of [path.join('target', 'release', 'cpm.exe'), path.join('target', 'debug', 'cpm.exe')]) {
                const p = path.join(curr, rel);
                if (fs.existsSync(p)) {
                    return p;
                }
            }
            const parent = path.dirname(curr);
            if (parent === curr) break;
            curr = parent;
        }
        return 'cpm';
    }

    /**
     * Call a foreign language method over CPM stdio RPC bridge.
     * @param {string} target Method target in format 'language:method' (e.g. 'python:math.sqrt')
     * @param {Array} args Arguments array
     * @returns {Promise<any>} Response result from foreign runtime
     */
    call(target, args = []) {
        return new Promise((resolve, reject) => {
            const argsJson = JSON.stringify(args);
            execFile(this.cpmBin, ['bridge', 'call', target, argsJson], { encoding: 'utf8' }, (err, stdout, stderr) => {
                if (err) {
                    return reject(new Error(`CPM Bridge Error: ${stderr.trim() || stdout.trim()}`));
                }

                const lines = stdout.trim().split('\n');
                let capture = false;
                const jsonLines = [];

                for (const line of lines) {
                    if (line.includes('Response received:')) {
                        capture = true;
                        continue;
                    }
                    if (capture && (line.includes('round-trip via stdio RPC') || !line.trim())) {
                        if (jsonLines.length > 0) break;
                    }
                    if (capture) {
                        jsonLines.push(line);
                    }
                }

                const rawJson = jsonLines.join('\n').trim();
                if (rawJson) {
                    try {
                        return resolve(JSON.parse(rawJson));
                    } catch (pErr) {
                        return resolve(rawJson);
                    }
                }
                resolve(stdout.trim());
            });
        });
    }

    /**
     * Inspect registered methods on a foreign language host.
     * @param {string} language Language name (python, node)
     * @returns {Promise<string>} Output method list
     */
    inspect(language) {
        return new Promise((resolve, reject) => {
            execFile(this.cpmBin, ['bridge', 'inspect', language], { encoding: 'utf8' }, (err, stdout, stderr) => {
                if (err) return reject(err);
                resolve(stdout.trim());
            });
        });
    }
}

if (require.main === module) {
    (async () => {
        const bridge = new CpmBridge();
        console.log('Testing CPM Node.js SDK:');
        try {
            const res = await bridge.call('python:math.sqrt', [144.0]);
            console.log('  sqrt(144.0) =', res);
        } catch (e) {
            console.error('  SDK Error:', e.message);
        }
    })();
}

module.exports = { CpmBridge };
