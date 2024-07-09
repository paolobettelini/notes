# Notes

Welcome to my notes! The following is a collection of math notes
and related subjects.
These notes are organized using the [stellar](https://github.com/paolobettelini/stellar)
software.

# Website preview

![universe preview](./media/universe.png)
![course preview](./media/physicalrendering.png)

# How to run
Install [stellar](https://github.com/paolobettelini/stellar)
and clone the repository
```bash
git clone https://github.com/paolobettelini/notes
```
Set the necessary enviromental variables:
```bash
NOTESPATH # pointing to the notes/ directory
MONGO_CONNECTION_URL # connection URL to mongodb
```
Install the required libraries (some snippets require `npm` and `wasm-pack` to compile)
```bash
pacman -S tectonic npm wasm-pack git
```
Install the compiler (or download it from the releases)
```bash
cd notes/compiler
cargo build --release
mv target/release/compile /usr/local/bin/notes
```
# Compile everything
```bash
cd source
notes # compiles everything
cd ..
```

It is advisable to set the `CARGO_TARGET_DIR` variable so that
the rust projects share the same target folder.

Then, start the web server
```bash
stellar web --data data/ --connection-url $MONGO_CONNECTION_URL
```
Go to [localhost:8080/search](http://localhost:8080/search).

# Compiler usage
```bash
notes # compiles everything
notes Something # compiles everything that contains "Something"
notes -r "S|T|N|G" # compiles everything that matches the regex
notes --latex # compiles all the latex files
notes --snippets # compiles all the universes
notes --pages # compiles all the universes
notes --courses # compiles all the universes
notes --universes # compiles all the universes
notes --universes -r "..." # compiles all the universes that match the regex
```

# Fish configuration
If you're writing notes using stellar you might find this setup useful (`~/.config/fish/config.fish`):
```bash
set --export MONGO_CONNECTION_URL "mongodb://127.0.0.1" # modify accordingly
set --export NOTES_PATH "/home/user/notes" # modify accordingly

function startstellar
  sudo -v
  sudo stellar web --data "$NOTES_PATH/data" --connection-url $MONGO_CONNECTION_URL --port 80
end

function stopstellar
  sudo -v
  set pid (sudo lsof -t -i :80)
  if test -z "$pid"
    echo "Server was not open"
  else
    sudo kill -9 $pid
    echo "Server was killed"
  end
end

function updatenotes
  sudo -v
  set latest_url (curl -s https://api.github.com/repos/paolobettelini/notes/releases/latest | grep browser_download_url | cut -d '"' -f 4)
  set temp_file /tmp/notes_latest
  curl -L -o $temp_file $latest_url
  chmod +x $temp_file
  sudo mv $temp_file /usr/local/bin/notes
end

function updatestellar
  sudo -v
  set latest_url (curl -s https://api.github.com/repos/paolobettelini/stellar/releases/latest | grep browser_download_url | cut -d '"' -f 4)
  set temp_file /tmp/stellar_latest
  curl -L -o $temp_file $latest_url
  chmod +x $temp_file
  sudo mv $temp_file /usr/local/bin/stellar
end
```

# Cite me
```bib
@online{paolobettelini,
  author    = {Paolo Bettelini},
  title     = {notes},
  year      = {2024},
  publisher = {GitHub},
  journal   = {GitHub repository}
}
```