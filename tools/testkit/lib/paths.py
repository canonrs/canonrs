#!/usr/bin/env python3
"""
paths.py — Utilitario de paths para canonrs testkit
Detecta automaticamente se esta no CI (GITHUB_WORKSPACE) ou local
"""
import os

def canonrs_root():
    """Retorna o root do repositorio canonrs."""
    return os.environ.get(
        'GITHUB_WORKSPACE',
        '/opt/docker/monorepo/packages-rust/rs-canonrs'
    )

CANONRS_DIR   = canonrs_root()
PRIMITIVES_DIR = f"{CANONRS_DIR}/canonrs-core/src/primitives"
UI_DIR         = f"{CANONRS_DIR}/canonrs-server/src/ui"
TESTKIT_DIR    = f"{CANONRS_DIR}/tools/testkit"
