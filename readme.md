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

## Example usages

Invoker key mapping:

```json
{
  "global_delay": 50,
  "mappings": {
    "F1": [
      { "key": "q", "delay": 0, "direction": "Click" }, // Invoke Cold Snap (Q-Q-Q-R)
      { "key": "q", "delay": 50, "direction": "Click" },
      { "key": "q", "delay": 50, "direction": "Click" },
      { "key": "r", "delay": 50, "direction": "Click" },
      { "key": "d", "delay": 100, "direction": "Click" } // Cast Cold Snap (D)
    ],
    "F2": [
      { "key": "w", "delay": 0, "direction": "Click" }, // Invoke EMP (W-W-W-R)
      { "key": "w", "delay": 50, "direction": "Click" },
      { "key": "w", "delay": 50, "direction": "Click" },
      { "key": "r", "delay": 50, "direction": "Click" },
      { "key": "d", "delay": 100, "direction": "Click" } // Cast EMP (D)
    ],
    "F3": [
      { "key": "e", "delay": 0, "direction": "Click" }, // Invoke Sun Strike (E-E-E-R)
      { "key": "e", "delay": 50, "direction": "Click" },
      { "key": "e", "delay": 50, "direction": "Click" },
      { "key": "r", "delay": 50, "direction": "Click" },
      { "key": "d", "delay": 100, "direction": "Click" } // Cast Sun Strike (D)
    ],
    "F4": [
      { "key": "w", "delay": 0, "direction": "Click" }, // Invoke Tornado (W-Q-W-R)
      { "key": "q", "delay": 50, "direction": "Click" },
      { "key": "w", "delay": 50, "direction": "Click" },
      { "key": "r", "delay": 50, "direction": "Click" },
      { "key": "d", "delay": 100, "direction": "Click" } // Cast Tornado (D)
    ],
    "F5": [
      { "key": "q", "delay": 0, "direction": "Click" }, // Invoke Ice Wall (Q-Q-E-R)
      { "key": "q", "delay": 50, "direction": "Click" },
      { "key": "e", "delay": 50, "direction": "Click" },
      { "key": "r", "delay": 50, "direction": "Click" },
      { "key": "f", "delay": 100, "direction": "Click" } // Cast Ice Wall (F)
    ],
    "F6": [
      { "key": "w", "delay": 0, "direction": "Click" }, // Invoke Alacrity (W-W-E-R)
      { "key": "w", "delay": 50, "direction": "Click" },
      { "key": "e", "delay": 50, "direction": "Click" },
      { "key": "r", "delay": 50, "direction": "Click" },
      { "key": "f", "delay": 100, "direction": "Click" } // Cast Alacrity (F)
    ],
    "F7": [
      { "key": "q", "delay": 0, "direction": "Click" }, // Invoke Ghost Walk (Q-Q-W-R)
      { "key": "q", "delay": 50, "direction": "Click" },
      { "key": "w", "delay": 50, "direction": "Click" },
      { "key": "r", "delay": 50, "direction": "Click" },
      { "key": "v", "delay": 100, "direction": "Click" } // Cast Ghost Walk (V)
    ]
  }
}
```
