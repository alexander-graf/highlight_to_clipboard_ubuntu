# highlight_to_clipboard_ubuntu

Program Name
The program is called "highlight_to_clipboard".

Program Description
The program saves highlighted text to a file and displays a tooltip with the content. It also generates a random 6-digit number and creates a file with the content in a specified folder. it is my obsidian path. choose whatever you like.

Dependencies
The following dependencies are needed to run the program:

rand: used for generating random numbers
shellexpand: used for expanding the base folder path
xsel: used for accessing the clipboard
kdialog: used for displaying tooltips
To add these dependencies to your Cargo.toml file, you can use the following code:


dependencies

rand = "0.8"
shellexpand = "2.0"


on ubuntu you also need kdialog. 

Thats what happens:

Wait for 1 second.
Capture the content of text.
Display a tooltip with the message "Gestartet."
Generate a random 6-digit number.
Create a file with the prefix "snip" and the random number in the specified folder.
Save the content of the clipboard to the created file.
Print the content of the clipboard.
Print the path of the created file.
Display a tooltip with the content of the clipboard.
Troubleshooting
If the length of the content in the clipboard is 0, the program will exit with an error.

If the execution of the xsel command or the kdialog command fails, an error message will be printed.

Make sure the necessary dependencies are installed and accessible in the system's PATH.

License
This program is licensed under the i don't care license.
