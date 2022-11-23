import jsonschema

schema = {
    "type": "object",
    "properties": {
        "some_text": {
            "type": "string",
            "minLength": 10
        },
        "some_number": {
            "type": "number",
            "max": 100
        }
    },
    "required": ["some_text", "some_number"],
    "additionalProperties": False
}


correct_data = {"some_text": "abc def ghi jkl", "some_number": 42}
failing_limits = {"some_text": "abc", "some_number": 420}
completely_wrong = {"abc": "abc", "number": 4200}


jsonschema.validate(correct_data, schema)
try:
    jsonschema.validate(failing_limits, schema)
except jsonschema.ValidationError as VE:
    assert all(i in VE.args[0] for i in ["abc", "too short"])

try:
    jsonschema.validate(completely_wrong, schema)
except jsonschema.ValidationError as VE:
    assert all(i in VE.args[0] for i in ["some_text", "required property"])
