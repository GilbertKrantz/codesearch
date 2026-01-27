"""
Data processor module with various code patterns
Demonstrates: complexity, dead code, code smells
"""

# TODO: Add support for streaming data
# FIXME: Memory inefficient for large datasets

from typing import List, Dict, Optional, Any
import json

# Unused import - dead code
from collections import defaultdict


class DataProcessor:
    """Process and transform data with various patterns"""

    # Magic number
    MAX_SIZE = 1000

    def __init__(self, config: Optional[Dict] = None):
        # TODO: Load configuration from file
        self.config = config or {}
        self.cache: Dict[str, Any] = {}

    # Unused method - dead code
    def _legacy_method(self):
        """This is no longer used"""
        return "legacy"

    def process_items(self, items: List[Dict]) -> List[Dict]:
        """Process list of items - high complexity"""
        results = []

        for item in items:
            # Deep nesting level 1
            if 'value' in item:
                # Deep nesting level 2
                if item['value'] > 0:
                    # Deep nesting level 3
                    if 'type' in item:
                        # Deep nesting level 4
                        if item['type'] == 'special':
                            # Deep nesting level 5
                            if 'priority' in item:
                                results.append({
                                    'processed': True,
                                    'value': item['value'] * 2,
                                    'type': item['type']
                                })
                            else:
                                results.append({
                                    'processed': True,
                                    'value': item['value'] * 2,
                                    'type': item['type']
                                })
                        else:
                            results.append({
                                'processed': True,
                                'value': item['value']
                            })
                    else:
                        results.append({
                            'processed': True,
                            'value': item['value']
                        })
                else:
                    results.append({
                        'processed': False,
                        'value': item['value']
                    })
            else:
                results.append({
                    'processed': False,
                    'error': 'No value'
                })

        return results

    def filter_data(self, data: List[Dict], filters: Dict) -> List[Dict]:
        """Filter data with multiple conditions - high complexity"""
        results = []

        for item in data:
            # Multiple conditions
            if 'status' in filters:
                if filters['status'] == 'active':
                    if item.get('active', False):
                        results.append(item)
                elif filters['status'] == 'inactive':
                    if not item.get('active', True):
                        results.append(item)
                elif filters['status'] == 'all':
                    results.append(item)
            else:
                results.append(item)

        return results

    def transform_data(self, data: Dict) -> Dict:
        """Transform data with multiple paths"""
        result = {}

        if 'name' in data:
            if isinstance(data['name'], str):
                if len(data['name']) > 0:
                    result['name'] = data['name'].upper()
                else:
                    result['name'] = 'UNKNOWN'
            else:
                result['name'] = str(data['name'])
        else:
            result['name'] = 'DEFAULT'

        if 'value' in data:
            if isinstance(data['value'], (int, float)):
                if data['value'] > 0:
                    result['value'] = data['value'] * 2
                else:
                    result['value'] = 0
            else:
                result['value'] = 0
        else:
            result['value'] = 0

        return result

    # Empty method - code smell
    def clear_cache(self):
        # TODO: Implement cache clearing
        pass

    # Unreachable code
    def validate_input(self, value: int) -> bool:
        if value > 10:
            return True
        else:
            return False

        # Unreachable
        print("This will never execute")
        return False

    # Unused variable
    def process_with_metrics(self, data: List) -> Dict:
        processed_count = 0
        error_count = 0
        start_time = None  # Never used
        batch_size = 100  # Never used

        return {
            'processed': processed_count,
            'errors': error_count
        }


# Duplicate code - Type 2 clone
def process_addition(items: List[int]) -> int:
    """Process addition with logging - duplicate pattern"""
    result = sum(items)
    print(f"Processing addition: {items} = {result}")
    if result > 1000:
        print("Large result detected")
    return result


# Duplicate code - Type 2 clone
def process_multiplication(items: List[int]) -> int:
    """Process multiplication with logging - duplicate pattern"""
    result = 1
    for item in items:
        result *= item
    print(f"Processing multiplication: {items} = {result}")
    if result > 1000:
        print("Large result detected")
    return result


# Commented out code
def deprecated_function(x: int) -> int:
    # Old implementation
    # result = x * 2
    # if result > 100:
    #     return result
    # return 0

    # New implementation
    return x * 3


class DataAnalyzer:
    """Analyze data patterns"""

    def __init__(self):
        self.metrics: Dict[str, int] = {}

    def calculate_statistics(self, numbers: List[float]) -> Dict[str, float]:
        """Calculate statistics - redundant code"""
        total = sum(numbers)
        count = len(numbers)

        if count == 0:
            return {
                'mean': 0.0,
                'total': 0.0
            }

        # Redundant calculation
        mean = total / count
        total = sum(numbers)  # Re-calculated

        return {
            'mean': mean,
            'total': total
        }

    # Unused class method - dead code
    def _internal_method(self):
        return "internal"


# Unused global variable
DEBUG_MODE = True

# Another unused variable
CONFIG_PATH = "/etc/config.json"


def main():
    """Main function"""
    processor = DataProcessor()
    data = [
        {'value': 10, 'type': 'special', 'priority': 'high'},
        {'value': 20, 'type': 'normal'},
        {'value': -5, 'type': 'special'}
    ]

    results = processor.process_items(data)
    print(f"Processed {len(results)} items")


if __name__ == "__main__":
    main()
