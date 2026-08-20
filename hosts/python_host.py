import sys
import json
import struct
import math
import hashlib
import base64
import traceback

def read_message():
    header = sys.stdin.buffer.read(4)
    if not header or len(header) < 4:
        return None
    length = struct.unpack(">I", header)[0]
    body = sys.stdin.buffer.read(length)
    return json.loads(body.decode("utf-8"))

def send_message(msg):
    data = json.dumps(msg).encode("utf-8")
    header = struct.pack(">I", len(data))
    sys.stdout.buffer.write(header + data)
    sys.stdout.buffer.flush()

def handle_request(req):
    req_id = req.get("id")
    method = req.get("method")
    args = req.get("args", [])
    kwargs = req.get("kwargs", {})

    try:
        if method == "__inspect__":
            methods = [
                {"name": "math.sqrt", "description": "Square root of a number", "args": ["number"]},
                {"name": "hash.sha256", "description": "Compute SHA-256 hash of a string or blob", "args": ["data"]},
                {"name": "docling.parse", "description": "Parse PDF document into structured sections", "args": ["filename"]},
                {"name": "echo", "description": "Echo back the first argument", "args": ["value"]},
                {"name": "ping", "description": "Ping/pong health check", "args": []},
            ]
            send_message({"type": "response", "id": req_id, "result": methods, "error": None})

        elif method == "math.sqrt":
            val = float(args[0])
            result = math.sqrt(val)
            send_message({"type": "response", "id": req_id, "result": result, "error": None})

        elif method == "hash.sha256":
            item = args[0]
            if isinstance(item, dict) and "$blob" in item:
                raw_bytes = base64.b64decode(item.get("data_base64", ""))
            elif isinstance(item, str):
                raw_bytes = item.encode("utf-8")
            else:
                raw_bytes = str(item).encode("utf-8")
            
            digest = hashlib.sha256(raw_bytes).hexdigest()
            send_message({"type": "response", "id": req_id, "result": digest, "error": None})

        elif method == "docling.parse":
            filename = args[0] if args else "document.pdf"
            parsed_doc = {
                "filename": filename,
                "pages": 28,
                "sections": ["Executive Summary", "The Problem", "System Design", "Risk Register"],
                "text": "Write in one language. Depend on all of them.",
                "transports_benchmark": {"rpc_us": 156, "embed_us": 0.88, "ffi_us": 0.88}
            }
            send_message({"type": "response", "id": req_id, "result": parsed_doc, "error": None})

        elif method == "echo":
            send_message({"type": "response", "id": req_id, "result": args[0] if args else None, "error": None})

        elif method == "ping":
            send_message({"type": "response", "id": req_id, "result": "pong", "error": None})

        else:
            send_message({
                "type": "response",
                "id": req_id,
                "result": None,
                "error": {
                    "error_type": "NotImplementedError",
                    "message": f"Method '{method}' not implemented in Python host",
                    "stack_trace": None
                }
            })
    except Exception as e:
        send_message({
            "type": "response",
            "id": req_id,
            "result": None,
            "error": {
                "error_type": type(e).__name__,
                "message": str(e),
                "stack_trace": traceback.format_exc()
            }
        })

def main():
    while True:
        try:
            msg = read_message()
            if msg is None:
                break
            msg_type = msg.get("type")
            if msg_type == "request":
                handle_request(msg)
            elif msg_type == "ping":
                send_message({"type": "pong", "id": msg.get("id")})
        except Exception:
            break

if __name__ == "__main__":
    main()
