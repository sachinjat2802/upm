"""
CPM FastAPI Extension — Native FastAPI Integration Helper

Usage:
    from fastapi import FastAPI, Depends
    from sdk.python.fastapi_cpm import get_cpm_bridge, CpmBridge

    app = FastAPI()

    @app.get("/sqrt/{number}")
    def calculate_sqrt(number: float, bridge: CpmBridge = Depends(get_cpm_bridge)):
        return {"result": bridge.call("python:math.sqrt", [number])}
"""

import os
import sys

sys.path.append(os.path.abspath(os.path.dirname(__file__)))
from cpm_sdk import CpmBridge

_bridge_instance = None

def get_cpm_bridge() -> CpmBridge:
    """FastAPI Dependency Provider for CpmBridge singleton."""
    global _bridge_instance
    if _bridge_instance is None:
        _bridge_instance = CpmBridge()
    return _bridge_instance
