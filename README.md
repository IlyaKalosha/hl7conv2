## About
This is a python library written in Rust.
### Scope

- convert hl7 medical format to json
- convert json to hl7 medical format (TODO)

## How to install
```
pip install hl7conv2
```

## Examples
### Load hl7 from file and convert to json
```
from hl7conv2 import Hl7Json

hl7_obj = Hl7Json.from_file("src/hl7_example.txt")
hl7_in_json_format = hl7_obj.hl7_json
print(hl7_in_json_format)
```
### Load hl7 from string and convert to json
```
from hl7conv2 import Hl7Json

hl7_string = """hl7 data in tripple brackets"""

hl7_obj = Hl7Json(hl7_string)
hl7_in_json_format = hl7_obj.hl7_json
print(hl7_in_json_format)
```
### Access string hl7 data
```
from hl7conv2 import Hl7Json

hl7_obj = Hl7Json.from_file("src/hl7_example.txt")
print(hl7_obj.hl7_string)
```