const http = require('http');
const fs = require('fs');
const path = require('path');
const { execFile } = require('child_process');

// Find CPM binary executable
function findCpmBin() {
    const rootDir = path.resolve(path.join(__dirname, '..'));
    for (const rel of [path.join('target', 'release', 'cpm.exe'), path.join('target', 'debug', 'cpm.exe')]) {
        const p = path.join(rootDir, rel);
        if (fs.existsSync(p)) {
            return p;
        }
    }
    return 'cpm';
}

const CPM_BIN = findCpmBin();

// Helper to call CPM bridge
function callCpmBridge(target, args = []) {
    return new Promise((resolve, reject) => {
        const argsJson = JSON.stringify(args);
        execFile(CPM_BIN, ['bridge', 'call', target, argsJson], { encoding: 'utf8' }, (err, stdout, stderr) => {
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

const PORT = 3000;

const server = http.createServer(async (req, res) => {
    // CORS headers
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'GET, POST, OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type');

    if (req.method === 'OPTIONS') {
        res.writeHead(204);
        res.end();
        return;
    }

    const parsedUrl = new URL(req.url, `http://localhost:${PORT}`);
    const pathname = parsedUrl.pathname;

    // REST API Endpoint 1: Parse RFP via Python Docling RPC
    if (pathname === '/api/parse-rfp' && req.method === 'POST') {
        let body = '';
        req.on('data', chunk => { body += chunk; });
        req.on('end', async () => {
            try {
                const payload = JSON.parse(body || '{}');
                const filename = payload.filename || 'enterprise_cloud_rfp.pdf';

                const start = Date.now();
                // Call Python docling document parser via CPM Bridge!
                const parsedResult = await callCpmBridge('python:docling.parse', [filename]);
                const elapsedMs = Date.now() - start;

                const response = {
                    status: 'success',
                    architecture: 'Polyglot stdio RPC (Node.js API → Python Ingestion Engine)',
                    elapsed_ms: elapsedMs,
                    data: parsedResult
                };

                res.writeHead(200, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify(response));
            } catch (err) {
                res.writeHead(500, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify({ status: 'error', message: err.message }));
            }
        });
        return;
    }

    // REST API Endpoint 2: Calculate Score via Python math.sqrt RPC
    if (pathname === '/api/calculate-score' && req.method === 'POST') {
        let body = '';
        req.on('data', chunk => { body += chunk; });
        req.on('end', async () => {
            try {
                const payload = JSON.parse(body || '{}');
                const val = parseFloat(payload.value) || 144.0;

                const start = Date.now();
                // Call Python math.sqrt via CPM Bridge!
                const scoreResult = await callCpmBridge('python:math.sqrt', [val]);
                const elapsedMs = Date.now() - start;

                const response = {
                    status: 'success',
                    input_value: val,
                    match_score_root: scoreResult,
                    compliance_percentage: Math.min(100, Math.round(scoreResult * 8.33)),
                    elapsed_ms: elapsedMs
                };

                res.writeHead(200, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify(response));
            } catch (err) {
                res.writeHead(500, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify({ status: 'error', message: err.message }));
            }
        });
        return;
    }

    // REST API Endpoint 3: Polyglot Status
    if (pathname === '/api/polyglot-status' && req.method === 'GET') {
        const response = {
            project: 'RFP Proposal Automation Platform',
            version: '1.0.0',
            primary_language: 'javascript',
            orchestration: 'CPM / UPM (Universal Package Platform)',
            ecosystems: [
                { name: 'pnpm', language: 'javascript', role: 'Web Server & REST API Gateway' },
                { name: 'uv', language: 'python', role: 'Document Ingestion & AI Parsing' },
                { name: 'cargo', language: 'rust', role: 'High-Speed Tokenizer & Hashing' }
            ]
        };
        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify(response));
        return;
    }

    // Serve Static UI (public/index.html)
    let filePath = path.join(__dirname, 'public', pathname === '/' ? 'index.html' : pathname);
    fs.readFile(filePath, (err, content) => {
        if (err) {
            res.writeHead(404, { 'Content-Type': 'text/plain' });
            res.end('404 Not Found');
        } else {
            const ext = path.extname(filePath);
            const contentType = ext === '.html' ? 'text/html' : ext === '.css' ? 'text/css' : 'text/plain';
            res.writeHead(200, { 'Content-Type': contentType });
            res.end(content);
        }
    });
});

server.listen(PORT, () => {
    console.log('');
    console.log('======================================================');
    console.log(`  🚀 RFP Proposal Platform running on http://localhost:${PORT}`);
    console.log('  Polyglot Architecture: Node.js API + Python Ingestion Engine');
    console.log('======================================================');
    console.log('');
});
