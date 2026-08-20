"""
CPM Python SDK — Native Python Client for CPM / UPM Polyglot Bridge RPC

Usage:
    from cpm_sdk import CpmBridge

    bridge = CpmBridge()
    result = bridge.call("node:crypto.sha256", ["hello world"])
    print(result)
"""

import json
import struct
import subprocess
import os

class CpmBridge:
    def __init__(self, cpm_bin=None):
        self.cpm_bin = cpm_bin or self._find_cpm_bin()

    def _find_cpm_bin(self):
        root_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
        for rel in [os.path.join("target", "release", "cpm.exe"), os.path.join("target", "debug", "cpm.exe")]:
            p = os.path.join(root_dir, rel)
            if os.path.exists(p):
                return p
        return "cpm"

    def call(self, target: str, args=None):
        """
        Call a foreign language method over CPM stdio RPC bridge.
        
        :param target: Method target in format 'language:method' (e.g. 'node:crypto.sha256')
        :param args: List of arguments to pass to the foreign method
        :return: Decoded return result from foreign runtime
        """
        args_json = json.dumps(args or [])
        cmd = [self.cpm_bin, "bridge", "call", target, args_json]
        
        res = subprocess.run(cmd, capture_output=True, text=True, encoding='utf-8', errors='replace')
        if res.returncode != 0:
            raise RuntimeError(f"CPM Bridge Call Error: {res.stderr.strip() or res.stdout.strip()}")
            
        # Parse output for JSON response
        lines = res.stdout.strip().split("\n")
        json_lines = []
        capture = False
        for line in lines:
            if "Response received:" in line:
                capture = True
                continue
            if capture and ("round-trip via stdio RPC" in line or not line.strip()):
                if json_lines:
                    break
            if capture:
                json_lines.append(line)

        raw_json = "\n".join(json_lines).strip()
        if raw_json:
            return json.loads(raw_json)
        return res.stdout.strip()

    def inspect(self, language: str):
        """Inspect registered methods on a foreign language host."""
        cmd = [self.cpm_bin, "bridge", "inspect", language]
        res = subprocess.run(cmd, capture_output=True, text=True, encoding='utf-8', errors='replace')
        return res.stdout.strip()


if __name__ == "__main__":
    bridge = CpmBridge()
    print("Testing CPM Python SDK:")
    try:
        res = bridge.call("python:math.sqrt", [144.0])
        print("  sqrt(144.0) =", res)
    except Exception as e:
        print("  SDK Error:", e)
