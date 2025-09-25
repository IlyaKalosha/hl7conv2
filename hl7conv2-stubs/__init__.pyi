"""Type stubs for hl7conv2 package."""

from typing import List, Dict, Union, Optional

class Hl7Json:
    """A Python class for converting HL7 messages to JSON format with validation.
    
    This class provides functionality to parse HL7 (Health Level 7) messages
    and convert them into a structured JSON-like format using Python dictionaries.
    Includes comprehensive validation capabilities that can be enabled or disabled.
    """
    
    def __init__(self, hl7_string: str, validation_enabled: Optional[bool] = None, strict_validation: Optional[bool] = None, escaping_enabled: Optional[bool] = None) -> None:
        """Creates a new Hl7Json instance from an HL7 message string.
        
        Args:
            hl7_string: The HL7 message string to parse
            validation_enabled: Optional validation setting. If None, defaults to False.
            strict_validation: Optional strict validation setting. If None, defaults to True.
            escaping_enabled: Optional escaping setting. If None, defaults to False.
        """
        ...
    
    @classmethod
    def from_file(cls, path: str, validation_enabled: Optional[bool] = None, strict_validation: Optional[bool] = None, escaping_enabled: Optional[bool] = None) -> "Hl7Json":
        """Creates a new Hl7Json instance from an HL7 message file.
        
        Args:
            path: The file path to the HL7 message file
            validation_enabled: Optional validation setting. If None, defaults to False.
            strict_validation: Optional strict validation setting. If None, defaults to True.
            escaping_enabled: Optional escaping setting. If None, defaults to False.
            
        Returns:
            A new Hl7Json instance containing the file contents
            
        Raises:
            IOError: If the file cannot be read
            ValueError: If the file is empty or validation fails
        """
        ...
    
    @property
    def hl7_string(self) -> str:
        """The original HL7 message string with normalized line endings.
        
        Returns:
            The HL7 message string
        """
        ...
    
    @property
    def validation_enabled(self) -> bool:
        """Whether validation is currently enabled.
        
        Returns:
            True if validation is enabled, False otherwise
        """
        ...
    
    @validation_enabled.setter
    def validation_enabled(self, value: bool) -> None:
        """Enable or disable validation.
        
        Args:
            value: True to enable validation, False to disable
        """
        ...
    
    @property
    def strict_validation(self) -> bool:
        """Whether strict validation mode is enabled.
        
        Returns:
            True if strict validation is enabled, False for lenient mode
        """
        ...
    
    @strict_validation.setter
    def strict_validation(self, value: bool) -> None:
        """Enable or disable strict validation mode.
        
        Args:
            value: True for strict validation, False for lenient mode
        """
        ...
    
    @property
    def escaping_enabled(self) -> bool:
        """Whether escaping is enabled during parsing.
        
        Returns:
            True if escaping is enabled, False otherwise
        """
        ...
    
    @escaping_enabled.setter
    def escaping_enabled(self, value: bool) -> None:
        """Enable or disable escaping during parsing.
        
        Args:
            value: True to enable escaping, False to disable
        """
        ...
    
    @property
    def hl7_json(self) -> List[Dict[str, str]]:
        """Converts the HL7 message to a JSON-like structure.
        
        Returns a list of dictionaries, where each dictionary represents an HL7 segment.
        The first field of each segment is stored with the key "segment_name",
        and subsequent fields are stored with numeric keys (1, 2, 3, etc.).
        Sub-fields (separated by ^) are stored with dot notation (e.g., "3.1", "3.2").
        
        If validation is enabled, the message will be validated before conversion.
        
        Returns:
            A list of dictionaries representing the parsed HL7 segments
            
        Raises:
            ValueError: If validation is enabled and the message is invalid
            
        Example:
            >>> hl7_obj = Hl7Json("MSH|^~\\&|ADT1|HOSPITAL")
            >>> json_data = hl7_obj.hl7_json
            >>> # Returns: [{"segment_name": "MSH", "1": "^~\\&", "2": "ADT1", "3": "HOSPITAL"}]
        """
        ...
    
    def validate(self, strict_mode: Optional[bool] = None, validate_required_fields: Optional[bool] = None) -> None:
        """Validates the HL7 message with optional custom settings.
        
        Args:
            strict_mode: Optional strict validation mode override. If None, uses instance setting.
            validate_required_fields: Optional required fields validation override. If None, uses default.
            
        Raises:
            ValueError: If the message is invalid according to the validation rules
        """
        ...
    

class JsonHl7:
    """A Python class for converting JSON data to HL7 format.
    
    This class provides functionality to convert structured JSON data
    back into HL7 (Health Level 7) message format.
    """
    
    def __init__(self, json_data: List[Dict[str, str]]) -> None:
        """Creates a new JsonHl7 instance from JSON data.
        
        Args:
            json_data: List of dictionaries representing HL7 segments
        """
        ...
    
    @classmethod
    def from_file(cls, path: str) -> "JsonHl7":
        """Creates a new JsonHl7 instance from a JSON file.
        
        Args:
            path: The file path to the JSON file
            
        Returns:
            A new JsonHl7 instance containing the file contents
            
        Raises:
            IOError: If the file cannot be read
            ValueError: If the file is empty or contains invalid JSON
        """
        ...
    
    @property
    def json_data(self) -> List[Dict[str, str]]:
        """The JSON data representing HL7 segments.
        
        Returns:
            List of dictionaries representing HL7 segments
        """
        ...
    
    @property
    def hl7_string(self) -> str:
        """Converts the JSON data to HL7 message format.
        
        Returns:
            The HL7 message string
            
        Example:
            >>> json_data = [{"segment_name": "MSH", "1": "^~\\&", "2": "ADT1"}]
            >>> json_hl7 = JsonHl7(json_data)
            >>> hl7_string = json_hl7.hl7_string
            >>> # Returns: "MSH|^~\\&|ADT1"
        """
        ...

# Module-level exports
__all__ = ["Hl7Json", "JsonHl7"]
