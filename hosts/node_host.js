const crypto = require('crypto');

let buffer = Buffer.alloc(0);

process.stdin.on('data', (chunk) => {
    buffer = Buffer.concat([buffer, chunk]);
    processBuffer();
});

function processBuffer() {
    while (buffer.length >= 4) {
        const length = buffer.readUInt32BE(0);
        if (buffer.length < 4 + length) {
            break;
        }
        const messageBytes = buffer.slice(4, 4 + length);
        buffer = buffer.slice(4 + length);
        try {
            const msg = JSON.parse(messageBytes.toString('utf8'));
            handleMessage(msg);
        } catch (err) {
            console.error('JSON parse error in node_host:', err);
        }
    }
}

function sendMessage(msg) {
    const data = Buffer.from(JSON.stringify(msg), 'utf8');
    const header = Buffer.alloc(4);
    header.writeUInt32BE(data.length, 0);
    process.stdout.write(Buffer.concat([header, data]));
}

function handleMessage(msg) {
    if (msg.type === 'request') {
        const reqId = msg.id;
        const method = msg.method;
        const args = msg.args || [];

        try {
            if (method === '__inspect__') {
                const methods = [
                    { name: 'sharp.resize', description: 'Resize image to target width and height', args: ['filename', 'width', 'height'] },
                    { name: 'crypto.sha256', description: 'Compute SHA-256 hash using Node.js crypto module', args: ['data'] },
                    { name: 'echo', description: 'Echo back argument', args: ['value'] },
                    { name: 'ping', description: 'Ping/pong health check', args: [] }
                ];
                sendMessage({ type: 'response', id: reqId, result: methods, error: null });
            } else if (method === 'sharp.resize') {
                const width = args[1] || 100;
                const height = args[2] || 100;
                const result = {
                    resized: true,
                    width: width,
                    height: height,
                    format: 'png',
                    bytes: 1024
                };
                sendMessage({ type: 'response', id: reqId, result: result, error: null });
            } else if (method === 'crypto.sha256') {
                const item = args[0];
                let inputBytes;
                if (item && item.$blob) {
                    inputBytes = Buffer.from(item.data_base64 || '', 'base64');
                } else if (typeof item === 'string') {
                    inputBytes = Buffer.from(item, 'utf8');
                } else {
                    inputBytes = Buffer.from(String(item), 'utf8');
                }
                const hash = crypto.createHash('sha256').update(inputBytes).digest('hex');
                sendMessage({ type: 'response', id: reqId, result: hash, error: null });
            } else if (method === 'echo') {
                sendMessage({ type: 'response', id: reqId, result: args[0], error: null });
            } else if (method === 'ping') {
                sendMessage({ type: 'response', id: reqId, result: 'pong', error: null });
            } else {
                sendMessage({
                    type: 'response',
                    id: reqId,
                    result: null,
                    error: {
                        error_type: 'NotImplementedError',
                        message: `Method '${method}' not implemented in Node.js host`,
                        stack_trace: null
                    }
                });
            }
        } catch (err) {
            sendMessage({
                type: 'response',
                id: reqId,
                result: null,
                error: {
                    error_type: err.name || 'Error',
                    message: err.message,
                    stack_trace: err.stack
                }
            });
        }
    } else if (msg.type === 'ping') {
        sendMessage({ type: 'pong', id: msg.id });
    }
}
