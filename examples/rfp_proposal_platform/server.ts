import * as http from 'http';
import * as fs from 'fs';
import * as path from 'path';
import { execFile } from 'child_process';

// Type Interfaces
export interface RfpParsePayload {
    filename?: string;
    budget?: string;
}

export interface ApiResponse<T = any> {
    status: 'success' | 'error';
    architecture?: string;
    elapsed_ms?: number;
    data?: T;
    message?: string;
}

export interface ScorePayload {
    value?: number;
}

export interface ScoreResponse {
    status: string;
    input_value: number;
    match_score_root: number;
    compliance_percentage: number;
    elapsed_ms: number;
}

export interface PolyglotStatusResponse {
    project: string;
    version: string;
    primary_language: string;
    orchestration: string;
    ecosystems: Array<{
        name: string;
        language: string;
        role: string;
    }>;
}

// Find CPM binary executable
function findCpmBin(): string {
    const rootDir = path.resolve(path.join(__dirname, '..'));
    for (const rel of [path.join('target', 'release', 'cpm.exe'), path.join('target', 'debug', 'cpm.exe')]) {
        const p = path.join(rootDir, rel);
        if (fs.existsSync(p)) {
            return p;
        }
    }
    return 'cpm';
}

const CPM_BIN: string = findCpmBin();

// Helper to call CPM bridge
function callCpmBridge<T = any>(target: string, args: any[] = []): Promise<T> {
    return new Promise((resolve, reject) => {
        const argsJson = JSON.stringify(args);
        execFile(CPM_BIN, ['bridge', 'call', target, argsJson], { encoding: 'utf8' }, (err, stdout, stderr) => {
            if (err) {
                return reject(new Error(`CPM Bridge Error: ${stderr.trim() || stdout.trim()}`));
            }

            const lines = stdout.trim().split('\n');
            let capture = false;
            const jsonLines: string[] = [];

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
                    return resolve(JSON.parse(rawJson) as T);
                } catch (pErr) {
                    return resolve(rawJson as unknown as T);
                }
            }
            resolve(stdout.trim() as unknown as T);
        });
    });
}

const PORT: number = 3000;

const server: http.Server = http.createServer(async (req: http.IncomingMessage, res: http.ServerResponse) => {
    // CORS headers
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'GET, POST, OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type');

    if (req.method === 'OPTIONS') {
        res.writeHead(204);
        res.end();
        return;
    }

    const parsedUrl = new URL(req.url || '/', `http://localhost:${PORT}`);
    const pathname: string = parsedUrl.pathname;

    // REST API Endpoint 1: Parse RFP via Python Docling RPC
    if (pathname === '/api/parse-rfp' && req.method === 'POST') {
        let body = '';
        req.on('data', (chunk: Buffer) => { body += chunk.toString(); });
        req.on('end', async () => {
            try {
                const payload: RfpParsePayload = JSON.parse(body || '{}');
                const filename: string = payload.filename || 'enterprise_cloud_rfp.pdf';

                const start: number = Date.now();
                // Call Python docling document parser via CPM Bridge!
                const parsedResult = await callCpmBridge('python:docling.parse', [filename]);
                const elapsedMs: number = Date.now() - start;

                const response: ApiResponse = {
                    status: 'success',
                    architecture: 'Polyglot stdio RPC (TypeScript Web Server → Python Ingestion Engine)',
                    elapsed_ms: elapsedMs,
                    data: parsedResult
                };

                res.writeHead(200, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify(response));
            } catch (err: any) {
                res.writeHead(500, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify({ status: 'error', message: err.message }));
            }
        });
        return;
    }

    // REST API Endpoint 2: Calculate Score via Python math.sqrt RPC
    if (pathname === '/api/calculate-score' && req.method === 'POST') {
        let body = '';
        req.on('data', (chunk: Buffer) => { body += chunk.toString(); });
        req.on('end', async () => {
            try {
                const payload: ScorePayload = JSON.parse(body || '{}');
                const val: number = parseFloat(String(payload.value)) || 144.0;

                const start: number = Date.now();
                // Call Python math.sqrt via CPM Bridge!
                const scoreResult = await callCpmBridge<number>('python:math.sqrt', [val]);
                const elapsedMs: number = Date.now() - start;

                const response: ScoreResponse = {
                    status: 'success',
                    input_value: val,
                    match_score_root: scoreResult,
                    compliance_percentage: Math.min(100, Math.round(scoreResult * 8.33)),
                    elapsed_ms: elapsedMs
                };

                res.writeHead(200, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify(response));
            } catch (err: any) {
                res.writeHead(500, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify({ status: 'error', message: err.message }));
            }
        });
        return;
    }

    // REST API Endpoint 3: Polyglot Status
    if (pathname === '/api/polyglot-status' && req.method === 'GET') {
        const response: PolyglotStatusResponse = {
            project: 'RFP Proposal Automation Platform (TypeScript Edition)',
            version: '1.0.0',
            primary_language: 'typescript',
            orchestration: 'CPM / UPM (Universal Package Platform)',
            ecosystems: [
                { name: 'pnpm', language: 'typescript', role: 'TypeScript Web Server & REST API Gateway' },
                { name: 'uv', language: 'python', role: 'Document Ingestion & AI Parsing' },
                { name: 'cargo', language: 'rust', role: 'High-Speed Tokenizer & Hashing' }
            ]
        };
        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify(response));
        return;
    }

    // Serve Static UI (public/index.html)
    const filePath: string = path.join(__dirname, 'public', pathname === '/' ? 'index.html' : pathname);
    fs.readFile(filePath, (err: NodeJS.ErrnoException | null, content: Buffer) => {
        if (err) {
            res.writeHead(404, { 'Content-Type': 'text/plain' });
            res.end('404 Not Found');
        } else {
            const ext: string = path.extname(filePath);
            const contentType: string = ext === '.html' ? 'text/html' : ext === '.css' ? 'text/css' : 'text/plain';
            res.writeHead(200, { 'Content-Type': contentType });
            res.end(content);
        }
    });
});

server.listen(PORT, () => {
    console.log('');
    console.log('======================================================');
    console.log(`  🚀 RFP Proposal Platform (TypeScript Edition) on http://localhost:${PORT}`);
    console.log('  Polyglot Architecture: TypeScript Server + Python Ingestion Engine');
    console.log('======================================================');
    console.log('');
});
