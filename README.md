# sentinel
Deletes unnecessary files from the specified folder.

This App is designed to work in the background.
All settings are stored in `config.ini` along with the Sentinel main file.

Config.ini example

    [system]  
    path = path\to\monitored\folder  
    #polling period in seconds
    period = 5  

    [extensions]  
    #files that should remain in the folder
    excluded = png,jpg
