#!/usr/bin/env python3
"""
Basic usage examples for hl7conv2 library.
"""

from hl7conv2 import Hl7Json, JsonHl7

def main():
    print("=== HL7Conv2 Basic Usage Examples ===\n")
    
    # Example 1: HL7 to JSON conversion
    print("1. HL7 to JSON Conversion:")
    hl7_string = """MSH|^~\\&|ADT1|GOOD HEALTH HOSPITAL|GHH LAB, INC.|GOOD HEALTH HOSPITAL|198808181126|SECURITY|ADT^A01^ADT_A01|MSG00001|T|2.5.1
EVN||200708181123||
PID|1||PATID1234^5^M11^ADT1^MR^GOOD HEALTH HOSPITAL||EVERYMAN^ADAM^A^III||19610615|M||2106-3|2222 HOME STREET^^GREENSBORO^NC^27401-1020"""
    
    hl7_obj = Hl7Json(hl7_string)
    json_data = hl7_obj.hl7_json
    print("✅ Successfully converted HL7 to JSON")
    print(f"Number of segments: {len(json_data)}")
    print(f"First segment: {json_data[0]['segment_name']}")
    print()
    
    # Example 2: JSON to HL7 conversion
    print("2. JSON to HL7 Conversion:")
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
    print("✅ Successfully converted JSON to HL7")
    print("Generated HL7:")
    print(hl7_string)
    print()
    
    # Example 3: Bidirectional conversion
    print("3. Bidirectional Conversion Test:")
    original_hl7 = "MSH|^~\\&|ADT1|HOSPITAL|LAB|HOSPITAL|20240101120000|SECURITY|ADT^A01^ADT_A01|MSG00001|T|2.5.1"
    
    # HL7 → JSON → HL7
    hl7_obj = Hl7Json(original_hl7)
    json_data = hl7_obj.hl7_json
    json_hl7 = JsonHl7(json_data)
    converted_hl7 = json_hl7.hl7_string
    
    print(f"Original:  {original_hl7}")
    print(f"Converted: {converted_hl7}")
    print(f"Match: {original_hl7 == converted_hl7}")
    print()
    
    # Example 4: Validation features
    print("4. Validation Features:")
    
    # Create instance with validation enabled
    valid_hl7 = Hl7Json("MSH|^~\\&|ADT1|HOSPITAL|LAB|HOSPITAL|20240101120000|SECURITY|ADT^A01^ADT_A01|MSG00001|T|2.5.1\nPID|1||PATID1234||DOE^JOHN||19800101|M")
    print(f"Validation enabled: {valid_hl7.validation_enabled}")
    print(f"Strict validation: {valid_hl7.strict_validation}")
    
    # Test with invalid message
    try:
        invalid_hl7 = Hl7Json("INVALID|SEGMENT")
        invalid_hl7.validation_enabled = True
        invalid_hl7.validate()
        print("❌ Should have failed validation")
    except Exception as e:
        print(f"✅ Correctly caught validation error: {e}")
    
    # Disable validation for invalid message
    invalid_hl7.validation_enabled = False
    json_data = invalid_hl7.hl7_json
    print(f"✅ Successfully processed invalid message with validation disabled: {len(json_data)} segments")
    
    # Example 5: File operations
    print("\n5. File Operations:")
    try:
        # Load from HL7 file
        hl7_obj = Hl7Json.from_file("examples/hl7_example.txt")
        print("✅ Successfully loaded HL7 from file")
        print(f"File contains {len(hl7_obj.hl7_json)} segments")
        
        # Load from JSON file
        json_hl7 = JsonHl7.from_file("examples/json_example.json")
        print("✅ Successfully loaded JSON from file")
        print(f"File contains {len(json_hl7.json_data)} segments")
        
    except Exception as e:
        print(f"❌ File operation error: {e}")
    
    print("\n=== Examples completed successfully! ===")

if __name__ == "__main__":
    main()
