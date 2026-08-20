"""
CPM AWS Lambda Serverless Adapter — Cold-Start Optimized Polyglot Execution Layer

Usage in AWS Lambda handler:
    from sdk.lambda.cpm_lambda import CpmLambdaHandler

    handler = CpmLambdaHandler()

    def lambda_handler(event, context):
        return handler.dispatch(event)
"""

import json
import os
import sys

# Add parent SDK path
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "python")))
try:
    from cpm_sdk import CpmBridge
except ImportError:
    CpmBridge = None

class CpmLambdaHandler:
    def __init__(self):
        self.bridge = CpmBridge() if CpmBridge else None

    def dispatch(self, event):
        """Dispatch incoming Lambda API Gateway request to polyglot bridge handler."""
        path = event.get("path", "")
        method = event.get("httpMethod", "GET")
        
        if not self.bridge:
            return {"statusCode": 500, "body": json.dumps({"error": "CPM SDK not initialized"})}

        try:
            # Example polyglot delegation inside AWS Lambda
            res = self.bridge.call("python:math.sqrt", [144.0])
            return {
                "statusCode": 200,
                "headers": {"Content-Type": "application/json"},
                "body": json.dumps({
                    "message": "Processed by CPM Serverless Lambda Layer",
                    "path": path,
                    "method": method,
                    "result": res
                })
            }
        except Exception as e:
            return {"statusCode": 500, "body": json.dumps({"error": str(e)})}
