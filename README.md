# HL7Conv2
[![PyPI](https://img.shields.io/pypi/v/hl7conv2.svg)](https://pypi.org/project/hl7conv2/) [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A high-performance HL7 to JSON converter written in Rust with Python bindings, featuring comprehensive validation and escape sequence support.

## About
This is a Python library written in Rust that provides bidirectional conversion between HL7 (Health Level 7) medical format and JSON, with built-in validation capabilities and support for HL7 escape sequences. The JSON payload after conversion is compatible with Google's HL7 storage parser.

### Features

- **Bidirectional Conversion**: Convert HL7 to JSON and JSON to HL7
- **Built-in Validation**: Comprehensive HL7 message validation with strict/lenient modes
- **Escape Sequence Support**: Full support for HL7 escape sequences and special characters
- **Flexible Control**: Enable/disable validation and escaping independently
- **High Performance**: Written in Rust for optimal speed and memory efficiency
- **Error Handling**: Detailed error messages with context-specific information

## How to install
```
pip install hl7conv2
```

## Examples

### HL7 to JSON Conversion

#### Basic Usage (Validation Disabled by Default)
```python
from hl7conv2 import Hl7Json

# Load HL7 message from file (validation disabled by default)
hl7_obj = Hl7Json.from_file("examples/hl7_example.txt")
json_data = hl7_obj.hl7_json
print(json_data)

# Load from file with custom settings
hl7_obj = Hl7Json.from_file(
    "examples/hl7_example.txt",
    validation_enabled=True,
    strict_validation=False,
    escaping_enabled=True
)

# Check validation settings
print(f"Validation enabled: {hl7_obj.validation_enabled}")
print(f"Strict validation: {hl7_obj.strict_validation}")
print(f"Escaping enabled: {hl7_obj.escaping_enabled}")
```

#### Load HL7 from string and convert to JSON
```python
from hl7conv2 import Hl7Json

hl7_string = """MSH|^~\\&|ADT1|HOSPITAL|LAB|HOSPITAL|20240101120000|SECURITY|ADT^A01^ADT_A01|MSG00001|T|2.5.1
PID|1||PATID1234||DOE^JOHN||19800101|M"""

# Basic usage (default settings)
hl7_obj = Hl7Json(hl7_string)
json_data = hl7_obj.hl7_json
print(json_data)

# Create with custom settings
hl7_obj = Hl7Json(
    hl7_string,
    validation_enabled=True,
    strict_validation=False,
    escaping_enabled=True
)
```

#### Custom Validation and Escaping Settings
```python
from hl7conv2 import Hl7Json

# Create with default settings and configure at runtime
hl7_obj = Hl7Json("MSH|^~\\&|ADT1|HOSPITAL|...")

# Configure validation and escaping
hl7_obj.validation_enabled = True
hl7_obj.strict_validation = False  # Use lenient validation
hl7_obj.escaping_enabled = True    # Enable escape sequence processing

# Validate with custom settings
hl7_obj.validate(strict_mode=False, validate_required_fields=True)
```

#### Process Invalid Messages
```python
from hl7conv2 import Hl7Json

# Process invalid HL7 messages without validation
hl7_obj = Hl7Json("INVALID|SEGMENT")
hl7_obj.validation_enabled = False
json_data = hl7_obj.hl7_json  # Processes without validation
```

#### Escape Sequence Handling
```python
from hl7conv2 import Hl7Json

hl7_obj = Hl7Json("MSH|^~\\&|ADT1|HOSPITAL")


# Check current settings
print(f"Escaping enabled: {hl7_obj.escaping_enabled}")
print(f"Validation enabled: {hl7_obj.validation_enabled}")
```

### JSON to HL7 Conversion

#### Load JSON from file and convert to HL7
```python
from hl7conv2 import JsonHl7

# Load JSON data from file
json_hl7 = JsonHl7.from_file("examples/json_example.json")
hl7_string = json_hl7.hl7_string
print(hl7_string)
```

#### Create JSON data programmatically and convert to HL7
```python
from hl7conv2 import JsonHl7

# Create JSON data representing HL7 segments
json_data = [
    {
        "segment_name": "MSH",
        "1": "^~\\&",
        "2": "ADT1",
        "3": "HOSPITAL",
        "4": "LAB",
        "5": "HOSPITAL",
        "6": "20240101120000",
        "7": "SECURITY",
        "8.1": "ADT",
        "8.2": "A01",
        "8.3": "ADT_A01",
        "9": "MSG00001",
        "10": "T",
        "11": "2.5.1"
    },
    {
        "segment_name": "PID",
        "1": "1",
        "3.1": "PATID1234",
        "3.2": "5",
        "3.3": "M11",
        "5.1": "DOE",
        "5.2": "JOHN",
        "7": "19800101",
        "8": "M"
    }
]

json_hl7 = JsonHl7(json_data)
hl7_string = json_hl7.hl7_string
print(hl7_string)
```

#### Access JSON data
```python
from hl7conv2 import JsonHl7

json_hl7 = JsonHl7.from_file("examples/json_example.json")
print(json_hl7.json_data)
```

### JSON Format Structure

The JSON format uses the following structure:

- **List of dictionaries**: Each dictionary represents an HL7 segment
- **`segment_name`**: Contains the segment type (MSH, PID, EVN, etc.)
- **Numeric keys**: Field positions (1, 2, 3, etc.)
- **Dot notation**: Field components (3.1, 3.2, 3.3 for field 3 components)
- **Empty strings**: Represent empty fields

#### Example JSON Structure:
```json
[
  {
    "segment_name": "MSH",
    "1": "^~\\&",
    "2": "ADT1",
    "3": "HOSPITAL",
    "8.1": "ADT",
    "8.2": "A01",
    "8.3": "ADT_A01"
  },
  {
    "segment_name": "PID",
    "1": "1",
    "3.1": "PATID1234",
    "3.2": "5",
    "3.3": "M11"
  }
]
```

This converts to:
```
MSH|^~\\&|ADT1|HOSPITAL|||||ADT^A01^ADT_A01
PID|1||PATID1234^5^M11
```

## Validation Features

### Built-in Validation

The `Hl7Json` class includes comprehensive validation capabilities:

#### Validation Modes
- **Strict Mode**: Full validation including HL7 version compatibility, message type format, and required segments
- **Lenient Mode**: Basic structure validation with optional required field validation

#### Properties and Settings
```python
# Check current settings
print(f"Validation enabled: {hl7_obj.validation_enabled}")
print(f"Strict validation: {hl7_obj.strict_validation}")
print(f"Escaping enabled: {hl7_obj.escaping_enabled}")
```

#### Validation Methods
```python
# Validate manually
try:
    hl7_obj.validate()
    print("Message is valid")
except ValueError as e:
    print(f"Validation error: {e}")

# Custom validation settings
hl7_obj.validate(
    strict_mode=True,
    validate_required_fields=True
)

# Enable/disable validation and escaping
hl7_obj.validation_enabled = True
hl7_obj.strict_validation = True
hl7_obj.escaping_enabled = True
```

### Error Handling

The library provides detailed error messages for various scenarios:
- **Validation Errors**: Specific validation failures with context
- **Parsing Errors**: Line-specific parsing issues
- **Field Errors**: Field-specific problems with segment and field information
- **Component Errors**: Component-specific issues with detailed location

## Bidirectional Conversion Example

```python
from hl7conv2 import Hl7Json, JsonHl7

# Original HL7 message
original_hl7 = "MSH|^~\\&|ADT1|HOSPITAL|LAB|HOSPITAL|20240101120000|SECURITY|ADT^A01^ADT_A01|MSG00001|T|2.5.1"

# HL7 → JSON → HL7
hl7_obj = Hl7Json(original_hl7)
json_data = hl7_obj.hl7_json
json_hl7 = JsonHl7(json_data)
converted_hl7 = json_hl7.hl7_string

print(f"Original:  {original_hl7}")
print(f"Converted: {converted_hl7}")
print(f"Match: {original_hl7 == converted_hl7}")
```

## API Reference

### Hl7Json Class

#### Constructors
- `Hl7Json(hl7_string, validation_enabled=None, strict_validation=None, escaping_enabled=None)` - Create with optional settings
- `Hl7Json.from_file(path, validation_enabled=None, strict_validation=None, escaping_enabled=None)` - Load from file with optional settings

#### Properties
- `hl7_string` - Original HL7 message string
- `validation_enabled` - Whether validation is enabled
- `strict_validation` - Whether strict validation mode is used
- `escaping_enabled` - Whether escaping is enabled during parsing
- `hl7_json` - Converted JSON data (triggers validation if enabled)

#### Methods
- `validate(strict_mode=None, validate_required_fields=None)` - Validate the message manually with optional custom settings

**Note:** Validation is lazy - it only occurs when explicitly called via `validate()` or when accessing the `hl7_json` property (if `validation_enabled=True`). Constructors do not perform automatic validation.

### JsonHl7 Class

#### Constructors
- `JsonHl7(json_data)` - Create from JSON data
- `JsonHl7.from_file(path)` - Load JSON from file

#### Properties
- `json_data` - Original JSON data
- `hl7_string` - Converted HL7 message string

## Development

This library is built with:
- **Rust** - Core conversion logic with high performance
- **PyO3** - Python bindings for seamless integration
- **Maturin** - Build system for Python extensions
- **Serde** - Fast serialization/deserialization
- **ThisError** - Comprehensive error handling

### Key Features
- **Performance Optimizations**: Serde integration and memory-efficient parsing
- **Escape Sequence Support**: Full HL7 escape sequence handling
- **Comprehensive Validation**: Built-in validation with configurable strictness
- **Error Handling**: Detailed, context-specific error messages
- **Type Safety**: Full Python type hints and stubs
