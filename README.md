# HL7Conv2

A high-performance HL7 to JSON converter written in Rust with Python bindings.

## About
This is a Python library written in Rust that provides bidirectional conversion between HL7 (Health Level 7) medical format and JSON.

### Scope

- Convert HL7 medical format to JSON
- Convert JSON data back to HL7 format

## How to install
```
pip install hl7conv2
```

## Examples

### HL7 to JSON Conversion

#### Load HL7 from file and convert to JSON
```python
from hl7conv2 import Hl7Json

# Load HL7 message from file
hl7_obj = Hl7Json.from_file("hl7_example.txt")
json_data = hl7_obj.hl7_json
print(json_data)
```

#### Load HL7 from string and convert to JSON
```python
from hl7conv2 import Hl7Json

hl7_string = """hl7 data in tripple brackets"""

hl7_obj = Hl7Json(hl7_string)
json_data = hl7_obj.hl7_json
print(json_data)
```

#### Access original HL7 string data
```python
from hl7conv2 import Hl7Json

hl7_obj = Hl7Json.from_file("src/hl7_example.txt")
print(hl7_obj.hl7_string)
```

### JSON to HL7 Conversion

#### Load JSON from file and convert to HL7
```python
from hl7conv2 import JsonHl7

# Load JSON data from file
json_hl7 = JsonHl7.from_file("json_example.json")
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

json_hl7 = JsonHl7.from_file("json_example.json")
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

## Development

This library is built with:
- **Rust** - Core conversion logic
- **PyO3** - Python bindings
- **Maturin** - Build system
