# KeyMastro

KeyMastro is a command-line tool designed to automate key presses and sequences for gaming or other repetitive tasks. It allows you to configure key mappings and delays using a JSON or YAML configuration file.

## Features

- Customizable key mappings.
- Support for delays between key presses.
- Configuration through JSON or YAML files.
- Logging for debugging and monitoring.

## Configuration

KeyMastro uses a JSON or YAML configuration file to define key mappings and delays.

### Configuration File Format

Here is an example configuration file in JSON format:

```json
{
  "global_delay": 50,
  "mappings": {
    "0": [
      { "key": "0", "delay": 100, "direction": "Click" },
      { "key": "Enter", "delay": 200, "direction": "Click" },
      { "key": "f", "delay": 150, "direction": "Click" }
    ]
  }
}
```

In YAML format:

```yaml
global_delay: 50
mappings:
  "0":
    - key: "0"
      delay: 100
      direction: "Click"
    - key: "Enter"
      delay: 200
      direction: "Click"
    - key: "f"
      delay: 150
      direction: "Click"
```

## Configuration Fields

- `global_delay` (optional): A global delay (in milliseconds) added before each key press.
- `mappings`: A dictionary where the keys are the key codes to listen for and the values are lists of key press configurations.
- Each key press configuration includes:
  - `key`: The key to be pressed.
  - `delay` (optional): A delay (in milliseconds) before pressing this key.
  - `direction` (optional): The direction of the key event (`Press`, `Release`, `Click`). Defaults to `Click`.
