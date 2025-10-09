from typing import List, Dict, Union, Optional

class Hl7Json:
    """A Python class for converting HL7 messages to JSON format with validation."""
    
    def __init__(self, hl7_string: str, validation_enabled: Optional[bool] = None, strict_validation: Optional[bool] = None, escaping_enabled: Optional[bool] = None) -> None:
        """Creates a new Hl7Json instance from an HL7 message string."""
        ...
    
    @classmethod
    def from_file(cls, path: str, validation_enabled: Optional[bool] = None, strict_validation: Optional[bool] = None, escaping_enabled: Optional[bool] = None) -> "Hl7Json":
        """Creates a new Hl7Json instance from an HL7 message file."""
        ...
    
    @property
    def hl7_string(self) -> str:
        """The original HL7 message string with normalized line endings."""
        ...
    
    @property
    def validation_enabled(self) -> bool:
        """Whether validation is currently enabled."""
        ...
    
    @validation_enabled.setter
    def validation_enabled(self, value: bool) -> None:
        """Enable or disable validation."""
        ...
    
    @property
    def strict_validation(self) -> bool:
        """Whether strict validation mode is enabled."""
        ...
    
    @strict_validation.setter
    def strict_validation(self, value: bool) -> None:
        """Enable or disable strict validation mode."""
        ...
    
    @property
    def escaping_enabled(self) -> bool:
        """Whether escaping is enabled during parsing."""
        ...
    
    @escaping_enabled.setter
    def escaping_enabled(self, value: bool) -> None:
        """Enable or disable escaping during parsing."""
        ...
    
    @property
    def hl7_json(self) -> List[Dict[str, str]]:
        """Converts the HL7 message to a JSON-like structure."""
        ...
    
    def validate(self, strict_mode: Optional[bool] = None, validate_required_fields: Optional[bool] = None) -> None:
        """Validates the HL7 message with optional custom settings."""
        ...


class JsonHl7:
    """A Python class for converting JSON data to HL7 format."""
    
    def __init__(self, json_data: List[Dict[str, str]]) -> None:
        """Creates a new JsonHl7 instance from JSON data."""
        ...
    
    @classmethod
    def from_file(cls, path: str) -> "JsonHl7":
        """Creates a new JsonHl7 instance from a JSON file."""
        ...
    
    @property
    def json_data(self) -> List[Dict[str, str]]:
        """The JSON data representing HL7 segments."""
        ...
    
    @property
    def hl7_string(self) -> str:
        """Converts the JSON data to HL7 message format."""
        ...

__all__ = ["Hl7Json", "JsonHl7"]