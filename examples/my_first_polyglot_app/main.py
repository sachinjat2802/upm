"""
My First Polyglot App — Powered by CPM (Cross-language Package Manager)
"""

import sys
import os

if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8")

# Import CPM Python SDK
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "sdk", "python")))
from cpm_sdk import CpmBridge

def main():
    print("======================================================")
    print("  🚀 My First Polyglot App — CPM Multi-Language RPC   ")
    print("======================================================")
    print()

    bridge = CpmBridge()

    # 1. Call Python math.sqrt
    sqrt_res = bridge.call("python:math.sqrt", [256.0])
    print(f"  🐍 [Python] math.sqrt(256.0) → {sqrt_res}")

    # 2. Call Node.js crypto.sha256
    hash_res = bridge.call("node:crypto.sha256", ["CPM Polyglot Rocks!"])
    print(f"  📦 [Node.js] crypto.sha256('CPM Polyglot Rocks!') → {hash_res}")

    # 3. Call Python docling document parser
    doc_res = bridge.call("python:docling.parse", ["architecture_report.pdf"])
    print(f"  🐍 [Python] docling.parse('architecture_report.pdf') → filename={doc_res.get('filename')}, pages={doc_res.get('pages')}")

    # 4. Call Node.js sharp image resize
    image_res = bridge.call("node:sharp.resize", ["logo.png", 640, 480])
    print(f"  📦 [Node.js] sharp.resize('logo.png', 640, 480) → {image_res}")

    print()
    print("  ✨ All cross-language RPC calls completed with ZERO boilerplate!")
    print("======================================================")

if __name__ == "__main__":
    main()
