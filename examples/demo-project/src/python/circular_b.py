"""
Circular dependency example - Module B
Demonstrates: circular imports with module A
"""

from .circular_a import process_a, validate_a, transform_a


def process_b(data: str) -> str:
    """Process in module B - calls module A"""
    print(f"Module B processing: {data}")

    # Circular call back to module A
    if validate_a(data):
        transformed = transform_a(data)
        return f"B-processed: {transformed}"
    else:
        return f"B-rejected: {data}"


def validate_b(data: str) -> bool:
    """Validate in module B - called by module A"""
    return len(data) > 5 and data.isalnum()


def transform_b(data: str) -> str:
    """Transform in module B"""
    return data.lower()


# Orchestration function that creates circular flow
def orchestrate_pipeline(data: str) -> str:
    """Orchestrate circular pipeline"""
    validated = validate_a(data)

    if validated:
        # Circular call to module A
        result_a = process_a(data)

        # Check result and potentially call back
        if "B-processed" in result_a:
            return result_a
        else:
            # Call module B again
            return process_b(data)
    else:
        return "Pipeline failed"
