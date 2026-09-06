## 🛡️ Sentinel

**Sentinel** is a lightweight background utility designed to keep your directories clean by automatically removing unwanted files based on your custom rules.


### `config.ini` Example

```ini
[system]
# The folder you want Sentinel to monitor
path = path\to\monitored\folder

# Polling period in seconds (e.g., 5 for every 5 seconds)
period = 60

# Set to 'once' to run the cleanup one time and exit
# mode = once

[extensions]
# List of file extensions you want to KEEP. 
# All other files will be deleted.
excluded = png, jpg, pdf, txt