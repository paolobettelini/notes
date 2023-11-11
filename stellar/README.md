## Math courses
Some of the notes of this repository are being migrated to
[stellar](https://github.com/paolobettelini/stellar).
PDF renders will eventually be available but the point of this is to have
a centralized database of notes, which shared pieces of information and the
possibility to include animations and interactive animations, everything available on the web.

## How to run
Install [stellar](https://github.com/paolobettelini/stellar)
and compile all the snippets.
```bash
cd stellar
./compile
```
This script uses `tectonic` to compile all the `tex` file and then runs `stellar-cli`
to generate the snippets and import them in the database.
To compile the `nannout` snippets you also need `wasm-pack` and `npm`.
Make sure to open the script and change your configurations. A mongoDB database is needed.
Then, open the web server
```bash
stellar-cli web --data data/ --connection-url "mongodb://localhost"
```
Go to [localhost:8080/universe/math](http://localhost:8080/universe/math).