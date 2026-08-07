# AppleScripts for Finder

Move the front Finder window to the next/previous sibling folder, as `cdnext`
and `cdprev` do in the shell.

| Script | What it does |
|---|---|
| [`next_folder.applescript`](next_folder.applescript) | Show the next sibling folder in the front Finder window |
| [`previous_folder.applescript`](previous_folder.applescript) | Show the previous sibling folder in the front Finder window |

The siblings of a folder are the child folders of its parent folder, sorted by
name. When the front window shows `~/Photos/2024-05`, `next_folder` moves it to
`~/Photos/2024-06`, without opening a new window.

## Requirements

- macOS, and the `sibling` command v3.0.0 or later.
  - The scripts tell the result by the exit status of the command, which the
    older versions did not return. Check it by `sibling --version`.
- The `sibling` command in `/opt/homebrew/bin`, `/usr/local/bin`, `/usr/bin`,
  or `/bin`. AppleScript runs a command with a minimal `PATH`, hence, these
  directories are searched by the `searchPath` property of the scripts.
  Edit the property if you installed the command elsewhere.

## Installation

### The script menu (simplest)

Compile the scripts into the scripts folder of Finder.

```bash
mkdir -p ~/Library/Scripts/Applications/Finder
osacompile -o ~/Library/Scripts/Applications/Finder/"Next Folder.scpt" next_folder.applescript
osacompile -o ~/Library/Scripts/Applications/Finder/"Previous Folder.scpt" previous_folder.applescript
```

Then show the script menu in the menu bar; open **Script Editor**, and turn on
**Settings → General → Show Script menu in menu bar**.

The scripts under `Applications/Finder` appear in the menu while Finder is the
frontmost application. Choose *Next Folder* to move the window.

### A keyboard shortcut (recommended for daily use)

Wrap the script in a Quick Action, and assign a key to it.

1. Open **Automator**, and create a new **Quick Action**.
2. Set *Workflow receives* to **no input**, and *in* to **Finder**.
3. Add the **Run Shell Script** action, and put the following line in it.

   ```bash
   osascript "$HOME/Library/Scripts/Applications/Finder/Next Folder.scpt"
   ```

4. Save it as `Next Folder`, and repeat for `Previous Folder`.
5. Open **System Settings → Keyboard → Keyboard Shortcuts… → Services →
   General**, and assign the keys, such as `⌃⌘→` and `⌃⌘←`.

Now the front Finder window walks through the sibling folders by the keys.

### Running without installing

```bash
osascript next_folder.applescript
```

## Permissions

The first run asks *“osascript” (or Automator) wants to control “Finder”*.
Allow it; otherwise the script fails with a permission error. The setting is in
**System Settings → Privacy & Security → Automation**.

## Customization

Both scripts share the same body, and differ only in the `traversingType`
property at the top of them.

```applescript
property traversingType : "next"
```

Set it to `previous`, `first`, `last`, or `random` to make your own script; for
example, copy `next_folder.applescript` to `random_folder.applescript` and set
the property to `random`.

## Behavior

- The window keeps its folder when no more sibling folder is found; the script
  tells it by a notification, such as *No more next folder.*
- Any other failure, such as a folder having no parent folder, shows an alert
  with the message of the `sibling` command.
- The script does nothing but an alert when no Finder window is open.
