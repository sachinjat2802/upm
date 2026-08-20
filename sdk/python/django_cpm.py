"""
CPM Django App Integration Middleware

Usage in Django settings.py:
    MIDDLEWARE = [
        ...
        'sdk.python.django_cpm.CpmDjangoMiddleware',
    ]
"""

import os
import sys

sys.path.append(os.path.abspath(os.path.dirname(__file__)))
from cpm_sdk import CpmBridge

class CpmDjangoMiddleware:
    def __init__(self, get_response):
        self.get_response = get_response
        self.bridge = CpmBridge()

    def __call__(self, request):
        request.cpm_bridge = self.bridge
        response = self.get_response(request)
        return response
