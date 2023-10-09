# require_rs

## Usage
```
/// Only binary name
require_cmd!("git");

/// Binary name and expected version
require_cmd!("git", "3.1");

/// Binary name, expected version and version option  
require_cmd!("git", "0.1", "--version");
```