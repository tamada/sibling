-- Open the previous sibling folder of the folder shown in the front Finder window.
--
-- The siblings of the folder are the child folders of its parent folder,
-- sorted by name; this script moves the front window to the previous one of them.
--
-- Requirements: the sibling command; see https://github.com/tamada/sibling
-- Installation: see the README.md in this directory.
--
-- Set the traversingType property to "next", "first", "last", or "random"
-- to traverse in another way.

property traversingType : "previous"

-- The sibling command is looked up in these directories, since "do shell
-- script" gives a minimal PATH which has no directory of Homebrew.
property searchPath : "/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin"

on run
	tell application "Finder"
		if (count of Finder windows) is 0 then
			display alert "sibling" message "No Finder window is open." as warning
			return
		end if
		set currentFolder to (target of front Finder window) as alias
	end tell

	set currentPath to POSIX path of currentFolder
	set theCommand to "export PATH=" & quoted form of searchPath & "; " & ¬
		"sibling --type " & traversingType & " -- " & quoted form of currentPath

	try
		set siblingPath to do shell script theCommand
	on error errorMessage number errorNumber
		-- The exit status of the sibling command becomes the error number;
		-- 1 means no more sibling folder, and the others mean an error.
		if errorNumber is 1 then
			display notification "No more " & traversingType & " folder." with title "sibling"
		else
			display alert "sibling" message errorMessage as warning
		end if
		return
	end try

	set siblingFolder to POSIX file siblingPath as alias
	tell application "Finder"
		set target of front Finder window to siblingFolder
		activate
	end tell
end run
