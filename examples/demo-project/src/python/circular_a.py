"""
Circular dependency example - Module A
Demonstrates: circular imports, call graph complexity
"""

from .circular_b import process_b, validate_b


def process_a(data: str) -> str:
    """Process in module A - calls module B"""
    print(f"Module A processing: {data}")

    # Circular call to module B
    if validate_b(data):
        return process_b(data)
    else:
        return f"A-processed: {data}"


def validate_a(data: str) -> bool:
    """Validate in module A - called by module B"""
    return len(data) > 0 and not data.startswith("invalid")


def transform_a(data: str) -> str:
    """Transform in module A"""
    return data.upper()
